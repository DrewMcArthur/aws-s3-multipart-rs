mod upload;
use upload::upload;

use aws_sdk_s3::Client;
use std::{env, error::Error};

const CHUNK_SIZE: u64 = 1024 * 1024 * 5;
const MAX_CHUNKS: u64 = 10000;

enum TransferMode {
    Upload,
    Download,
    RemoteCopy,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3);
    let from = &args[1];
    let to = &args[2];

    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let mode = get_transfer_mode(from, to).unwrap();

    match mode {
        TransferMode::Upload => upload(client, from, to).await,
        TransferMode::Download => download(client, from, to).await,
        TransferMode::RemoteCopy => remote_copy(client, from, to).await,
    }
    Ok(())
}

fn is_remote_path(path: &str) -> bool {
    path.starts_with("s3://")
}

fn get_transfer_mode(from: &str, to: &str) -> Result<TransferMode, Box<dyn Error>> {
    if !is_remote_path(from) && is_remote_path(to) {
        Ok(TransferMode::Upload)
    } else if is_remote_path(from) && !is_remote_path(to) {
        Ok(TransferMode::Download)
    } else if is_remote_path(from) && is_remote_path(to) {
        Ok(TransferMode::RemoteCopy)
    } else {
        Err("Invalid transfer mode".into())
    }
}

async fn download(client: Client, from: &str, to: &str) {
    todo!()
}

async fn remote_copy(client: Client, from: &str, to: &str) {
    todo!()
}

struct RemotePath {
    bucket_name: String,
    key: String,
}

fn split_remote_path(path: &str) -> RemotePath {
    let path = path.replace("s3://", "");
    let mut pieces = path.split("/").collect::<Vec<&str>>();
    return RemotePath {
        bucket_name: pieces.pop().unwrap().to_string(),
        key: pieces.join("/"),
    };
}
