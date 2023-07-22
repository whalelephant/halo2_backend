#![cfg(not(feature = "cosmwasm"))]
use crate::errors::{CRSError, Error};
use bytesize::ByteSize;
use std::env;

use reqwest::Client;

const G1_START: usize = 28;
const G2_START: usize = 28 + (5_040_000 * 64);
const G2_END: usize = G2_START + 128 - 1;

const TRANSCRIPT_URL_ENV_VAR: &str = "TRANSCRIPT_URL";
const TRANSCRIPT_URL_FALLBACK: &str =
    "http://aztec-ignition.s3.amazonaws.com/MAIN%20IGNITION/sealed/transcript00.dat";

pub async fn get_aztec_crs(points_needed: u32) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let g1_end = G1_START + ((points_needed as usize - 1) * 64) - 1;

    let g1_data = download(G1_START, g1_end).await?;
    let g2_data = download(G2_START, G2_END).await?;

    Ok((g1_data, g2_data))
}

async fn download(start: usize, end: usize) -> Result<Vec<u8>, CRSError> {
    let transcript_url = match env::var(TRANSCRIPT_URL_ENV_VAR) {
        Ok(url) => url,
        Err(_) => TRANSCRIPT_URL_FALLBACK.into(),
    };

    let client = Client::new();

    let request = client
        .get(&transcript_url)
        .header(reqwest::header::RANGE, format!("bytes={start}-{end}"))
        .build()
        .map_err(|source| CRSError::Request {
            url: transcript_url.to_string(),
            source,
        })?;
    let response = client
        .execute(request)
        .await
        .map_err(|source| CRSError::Fetch {
            url: transcript_url.to_string(),
            source,
        })?;
    let total_size = response.content_length().ok_or(CRSError::Length {
        url: transcript_url.to_string(),
    })?;

    println!(
        "\nDownloading the Ignite SRS ({})",
        ByteSize(total_size).to_string_as(false)
    );
    let crs_bytes = response
        .bytes()
        .await
        .map_err(|source| CRSError::Download { source })?;
    println!("Downloaded the SRS successfully!");

    Ok(crs_bytes.into())
}
