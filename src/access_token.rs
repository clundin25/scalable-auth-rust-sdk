#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused)]
#![allow(improper_ctypes)] // This is because of transitive go dependencies in the header file. Not
                           // due to the code we use

use anyhow::{anyhow, Result};

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_ulong};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[derive(Debug)]
pub struct AccessToken(String);

impl AccessToken {
    pub async fn from_uri(uri: &str, scopes: &str) -> Result<Self> {
        let token = fetch_token(uri, scopes)?;
        Ok(Self(token))
    }

    pub async fn raw_authorization_header(&self) -> Result<(String, String)> {
        Ok(("authorization".to_owned(), format!("Bearer {}", self.0)))
    }
}

fn fetch_token(uri: &str, scopes: &str) -> Result<String> {
    let uri = CString::new(uri)?;
    let scopes = CString::new(scopes)?;

    let token_size = query_token_size(&uri, &scopes)?;

    let mut token_buffer: Vec<c_char> = Vec::with_capacity(token_size);
    read_token(&uri, &scopes, &mut token_buffer)?;

    let token_string = unsafe { CStr::from_ptr(token_buffer.as_ptr()) }.to_str()?;
    Ok(token_string.to_owned())
}

fn query_token_size(uri: &CString, scopes: &CString) -> Result<usize> {
    let mut token_len: c_ulong = 0;
    let result = unsafe {
        CreateAccessToken(
            uri.as_ptr(),
            scopes.as_ptr(),
            std::ptr::null_mut(),
            &mut token_len,
        )
    };

    if result == 0 {
        return Err(anyhow!("Error calling AccessToken"));
    }

    Ok(token_len as usize)
}

fn read_token(uri: &CString, scopes: &CString, token_buf: &mut Vec<c_char>) -> Result<()> {
    let mut token_len: c_ulong = token_buf.capacity().try_into()?;
    let result = unsafe {
        CreateAccessToken(
            uri.as_ptr(),
            scopes.as_ptr(),
            token_buf.as_mut_ptr(),
            &mut token_len,
        )
    };

    if result == 0 {
        return Err(anyhow!("Error calling AccessToken"));
    }

    Ok(())
}
