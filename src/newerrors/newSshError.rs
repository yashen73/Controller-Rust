use thiserror::Error;

#[derive(Debug, Error)]
pub enum newSshError {

    #[error("Failed to unwrap Session : {0}")]
    UnwrapSession(String),

    #[error("SSH session is not connected")]
    NotConnected,

    #[error("Failed to create SSH Channel : {0}")]
    ChannelCreation(String),

    #[error("Failed to execute command : {0}")]
    CommandExecution(String),

    #[error("Failed to read command output : {0}")]
    OutputRead(String),

    #[error("Failed to close session: {0}")]
    CloseSession(String),

    #[error("Failed to open shell: {0}")]
    OpenShell(String),

    #[error("Failed to write Commands: {0}")]
    Write_All(String),

    #[error("Failed to flush command: {0}")]
    Flush_command(String),
}
