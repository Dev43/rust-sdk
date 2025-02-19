use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use bytes::Bytes;
use reqwest::{Response, Url};
use serde::Deserialize;

use crate::error::BundlrError;

pub async fn check_and_return<T: for<'de> Deserialize<'de>>(
    res: Result<Response, reqwest::Error>,
) -> Result<T, BundlrError>
where
    T: Default,
{
    match res {
        Ok(r) => {
            if !r.status().is_success() {
                let status = r.status();
                let text = r
                    .text()
                    .await
                    .expect("Could not get error text")
                    .replace('\"', "");
                let msg = format!("Status: {}:{:?}", status, text);
                return Err(BundlrError::ResponseError(msg));
            };
            Ok(r.json::<T>().await.unwrap_or_default())
        }
        Err(err) => Err(BundlrError::ResponseError(err.to_string())),
    }
}

pub async fn get_nonce(
    client: &reqwest::Client,
    url: &Url,
    address: String,
    currency: String,
) -> Result<u64, BundlrError> {
    let res = client
        .get(
            url.join(&format!(
                "/account/withdrawals/{}?address={}",
                currency, address
            ))
            .expect("Could not join url with /account/withdrawals/{}?address={}"),
        )
        .send()
        .await;
    check_and_return::<u64>(res).await
}

// Reads `length` bytes at `offset` within `file`
#[allow(clippy::uninit_vec)]
#[allow(clippy::unused_io_amount)]
pub fn read_offset(file: &mut File, offset: u64, length: usize) -> Result<Bytes, std::io::Error> {
    let mut b = Vec::with_capacity(length);
    unsafe { b.set_len(length) };
    file.seek(SeekFrom::Start(offset))?;

    b.fill(0);

    file.read(&mut b)?;
    Ok(b.into())
}
