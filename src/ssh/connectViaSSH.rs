use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;


use crate::{
    auth::{AuthService},
    db::{device_database::DeviceDatabase},
    apiResponse::{self, ApiResponse},
    config::Config,
    sshConnection::{ConnectDeviceSSH, ConnectDeviceRequest, ConnetedSDeviceResponse, DeviceCMD, DeviceRPLY}
};

pub async fn connecting_device(&self, command: DeviceCMD) -> Result<DeviceRPLY, Error>{
    let tcp = TcpStream::connect(command.ip_addr, command.port_no)?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    session.
}