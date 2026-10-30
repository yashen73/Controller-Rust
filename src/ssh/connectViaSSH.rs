use actix_web::{Error, HttpResponse, error, guard};
use thiserror::Error;
use actix_web::error::DispatchError::InternalError;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized};
use async_ssh2_tokio::Error::SshError;
use ssh2::{Channel, ErrorCode, Session};
use std::collections::HashMap;
use std::f32::consts::E;
use std::io::Read;
use std::process::CommandArgs;
use std::sync::Mutex;
use std::convert::TryFrom;
use std::net::{SocketAddr, TcpStream};
use std::ptr::null;


use crate::{
    auth::{AuthService},
    db::{device_database::DeviceDatabase},
    config::general_config,
    models::{
        apiResponse::{ApiResponse},
        sshConnection::{ConnectSSHRequest, ConnetedClosedSDeviceResponse, DeviceCMD, DeviceRPLY, CloseSSHRequest}
    },
    newerrors::newSshError,
};

pub struct Connect_Via_SSH{
    session: Mutex<Option<HashMap<String, Session>>>,
}

impl Connect_Via_SSH {
    
    pub fn new() -> Self {
        Self {
                session: Mutex::new(None),
        }
    }

    //Trying to connect with device using SSH
    pub async fn connecting_device(&self, request_to_connect: ConnectSSHRequest) -> Result<String, Error>{
        println!("\n\n::::::::: >>>>  Connecting_Device in connectViaSSH is called successfully  :::::::::");

        let ip = request_to_connect
        .ip_addr
        .as_ref()
        .ok_or_else(|| ErrorBadRequest("IP address is missing"))?
        .ip();

        let portno = u16::try_from(request_to_connect.port_no).map_err(|_| ErrorBadRequest("invalid port number"))?;
        let address = SocketAddr::new(ip, portno);
        
        let tcp = match TcpStream::connect(address){
            Ok(tcp) => tcp,
            Err(e) => {
                println!("Connection failed : {}", e);
                return Err(ErrorBadRequest(e.to_string()));
            }
        };

        let mut session = Session::new().map_err(|e| ErrorBadRequest(e.to_string()))?;
        session.set_tcp_stream(tcp);
        session.handshake();
        session.userauth_password(&request_to_connect.username, &request_to_connect.password);

        if !session.authenticated() {
           return Err(ErrorUnauthorized("Check hostname and password again".to_string()));
        }


        println!("::::::::: {} >>>>  Device Connected Successfully  :::::::::", request_to_connect.username);
        //store session inside stored_session
        let mut stored_session = self.session.lock().map_err(|_| ErrorInternalServerError("Session lock failed"))?;
        
        //Create HashMap for session to  catch them by their own usernames
        stored_session.get_or_insert_with(HashMap::new).insert(request_to_connect.username, session);
      
        println!(":::::::::  >>>>  Created Stored Channel Successfully  :::::::::");
        return Ok("success".to_string());

    }

    //Push_command is pushing Commands to the Device...
    pub async fn push_command(&self, command:DeviceCMD) -> Result<DeviceRPLY, newSshError::newSshError> {
        println!("\n\n::::::::: >>>>  Push_command in connectViaSSH is called successfully  :::::::::");
        
        //Grabbing stored sessions 
        let mut stored_session = self.session.lock().map_err(|e| newSshError::newSshError::UnwrapSession(e.to_string()))?;
        if stored_session.is_none() {
            print!("can not find stored_sesion");
        }
        //Grabbing tje session what we need from stored_session
        let mut new_stored_session = stored_session.as_ref().and_then(|m| m.get(&command.username)).cloned();
        if stored_session.is_none() {
            print!("can not find stored_sesion");
        }
        //Extracting the session for session element
        let session: &mut Session = match new_stored_session.as_mut(){
            Some(session) => session,
            None => {
                return Err(newSshError::newSshError::NotConnected);
            }
        };

        if stored_session.is_none() {
            print!("can not find stored_sesion");
        }

        let mut channel = session.channel_session().map_err(|e| newSshError::newSshError::ChannelCreation(e.to_string()))?;

        //channel.exec(&command.cmd).map_err(|e| ErrorInternalServerError(newSshError::newSshError::CommandExecution(e.to_string())));

        channel.request_pty("xterm-256color", None,  Some((120, 40, 0, 0)));

        if true {
            print!("Request pty is correct");
        }
        if let Err(e) = channel.shell(){
            return Err(
                newSshError::newSshError::OpenShell(e.to_string())
            );
        };

        // if let Err(e) = channel.write_all(command.cmd.as_bytes()){
        //     return Err(
        //         newSshError::newSshError::Write_All(e.to_string())
        //     );
        // };

        // if let Err(e) = channel.flush() {
        //     return Err(
        //         newSshError::newSshError::Flush_command(e.to_string())
        //     );
        // } 

        let mut output = String::new();

        if let Err(e) = channel.read_to_string(&mut output){ 
            return Err(
                newSshError::newSshError::OutputRead(e.to_string())
            );
        }

        channel.wait_close().map_err(|e: ssh2::Error| newSshError::newSshError::CloseSession(e.to_string()))?;
        
        if let Err(e) = channel.wait_close(){
            return Err(
                newSshError::newSshError::CloseSession(e.to_string())
            );
        }
        let rply = DeviceRPLY {
            id:  command.id,
            username: command.username,
            ip_addr: command.ip_addr,
            rply: output
        };
        return Ok(rply);
    }


    //Closing the Session 
    pub async fn closing_ssh_session(&self, command: CloseSSHRequest) -> Result<CloseSSHRequest, newSshError::newSshError>{
        println!("\n\n::::::::: >>>>  Closing_SSH_Session in connectViaSSH is called successfully  :::::::::");

       //Grabbing stored sessions 
        let mut stored_session = self.session.lock().map_err(|e| newSshError::newSshError::UnwrapSession(e.to_string()))?;
       
        let removed = stored_session.as_mut().map(|m| m.remove(&command.username)).flatten().is_some();

        if removed {
            return Ok(command);
        }else {
            println!("");
            return Err(newSshError::newSshError::CloseSession("can not close the session".to_string()));
        }
        
    }

}

