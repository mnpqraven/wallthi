use crate::{
    command::{
        Commands,
        state::{WallthiDaemon, WallthiStatus},
    },
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

    if let Some(status) = deserialize_bytes::<WallthiStatus>(&conn).await? {
        info!("{status:?}");
    }

    Ok(())
}

pub async fn process_cmd(stream: TcpStream, daemon: &WallthiDaemon) -> Result<(), AppError> {
    let cmd = deserialize_bytes::<Commands>(&stream).await?;

    if let Some(cmd) = cmd {
        info!("[COMMAND] RECEIVED {cmd:?}");
        daemon.handle_command(stream, cmd)?;
    };

    Ok(())
}

async fn deserialize_bytes<T: serde::de::DeserializeOwned>(
    stream: &TcpStream,
) -> Result<Option<T>, AppError> {
    let mut buffer = vec![0; 1024];
    stream.readable().await?;

    match stream.try_read(&mut buffer) {
        Ok(bytes_read) => {
            buffer.truncate(bytes_read);
            if buffer.is_empty() {
                return Ok(None);
            }

            let data = serde_json::from_slice::<T>(&buffer)?;
            Ok(Some(data))
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
        Err(e) => {
            panic!("{e}");
        }
    }
}
