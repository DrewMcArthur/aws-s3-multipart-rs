use std::path::Path;

use aws_sdk_s3::{
    operation::create_multipart_upload::CreateMultipartUploadOutput,
    types::{CompletedMultipartUpload, CompletedPart},
    Client,
};
use aws_smithy_http::byte_stream::{ByteStream, Length};

use crate::{split_remote_path, RemotePath, CHUNK_SIZE, MAX_CHUNKS};

pub async fn upload(client: Client, from: &str, to: &str) {
    let remote_destination = split_remote_path(to);
    let request = initiate_upload(&client, &remote_destination).await;
    let upload_id = request.upload_id().unwrap();
    let upload_parts = upload_file_chunks(&client, from, &remote_destination, &upload_id).await;
    finish_upload(&client, &remote_destination, upload_parts, upload_id).await;
}

async fn initiate_upload(
    client: &Client,
    remote_destination: &RemotePath,
) -> CreateMultipartUploadOutput {
    client
        .create_multipart_upload()
        .bucket(&remote_destination.bucket_name)
        .key(&remote_destination.key)
        .send()
        .await
        .unwrap()
}

async fn upload_file_chunks(
    client: &Client,
    path: &str,
    remote_destination: &RemotePath,
    upload_id: &str,
) -> Vec<CompletedPart> {
    let path = Path::new(&path);
    let file_size = tokio::fs::metadata(path)
        .await
        .expect("it exists I swear")
        .len();

    let mut chunk_count = (file_size / CHUNK_SIZE) + 1;
    let mut size_of_last_chunk = file_size % CHUNK_SIZE;
    if size_of_last_chunk == 0 {
        size_of_last_chunk = CHUNK_SIZE;
        chunk_count -= 1;
    }

    if file_size == 0 {
        panic!("Bad file size.");
    }
    if chunk_count > MAX_CHUNKS {
        panic!("Too many chunks! Try increasing your chunk size.")
    }

    let mut upload_parts: Vec<CompletedPart> = Vec::new();

    for chunk_index in 0..chunk_count {
        let this_chunk = if chunk_count - 1 == chunk_index {
            size_of_last_chunk
        } else {
            CHUNK_SIZE
        };
        let stream = ByteStream::read_from()
            .path(path)
            .offset(chunk_index * CHUNK_SIZE)
            .length(Length::Exact(this_chunk))
            .build()
            .await
            .unwrap();
        //Chunk index needs to start at 0, but part numbers start at 1.
        let part_number = (chunk_index as i32) + 1;
        // snippet-start:[rust.example_code.s3.upload_part]
        let upload_part_res = client
            .upload_part()
            .key(&remote_destination.key)
            .bucket(&remote_destination.bucket_name)
            .upload_id(upload_id)
            .body(stream)
            .part_number(part_number)
            .send()
            .await;
        upload_parts.push(
            CompletedPart::builder()
                .e_tag(upload_part_res.unwrap().e_tag.unwrap_or_default())
                .part_number(part_number)
                .build(),
        );
    }
    upload_parts
}

async fn finish_upload(
    client: &Client,
    remote_destination: &RemotePath,
    upload_parts: Vec<CompletedPart>,
    upload_id: &str,
) {
    let _complete_multipart_upload_res = client
        .complete_multipart_upload()
        .bucket(&remote_destination.bucket_name)
        .key(&remote_destination.key)
        .multipart_upload(
            CompletedMultipartUpload::builder()
                .set_parts(Some(upload_parts))
                .build(),
        )
        .upload_id(upload_id)
        .send()
        .await
        .unwrap();
}
