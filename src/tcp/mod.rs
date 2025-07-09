use crate::{
    command::{Commands, state::WallthiDaemon},
    utils::error::AppError,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tracing::info;

pub async fn send_cmd(cmd: Commands) -> Result<(), AppError> {
    info!("[COMMAND] EXECUTING {cmd:?}");
    let addr = WallthiDaemon::addr();
    let mut conn = TcpStream::connect(addr).await?;
    let bytes = serde_json::to_vec(&cmd)?;
    conn.write_all(&bytes).await?;
    Ok(())
}

pub async fn process_stream(stream: TcpStream, daemon: &WallthiDaemon) -> Result<(), AppError> {
    let mut buffer = vec![0; 1024];
    stream.readable().await?;

    match stream.try_read(&mut buffer) {
        Ok(bytes_read) => {
            buffer.truncate(bytes_read);
            // NOTE: giga important
            if buffer.is_empty() {
                // info!("EMPTY BUFFER");
                return Ok(());
            }

            let cmd = serde_json::from_slice::<Commands>(&buffer)?;
            info!("[COMMAND] RECEIVED {cmd:?}");
            daemon.handle_command(cmd)?;
            return Ok(());
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            return Ok(());
        }
        Err(e) => {
            panic!("{e}");
        }
    }
}
