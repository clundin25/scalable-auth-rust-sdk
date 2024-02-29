use anyhow::{anyhow, Result};

use core::ffi::c_int;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// #[link(name = "scalable-auth")]
extern "C" {
    fn CreateAccessToken(
        uri: *const c_char,
        scopes: *const c_char,
        token: *mut c_char,
        token_len: *mut c_int,
    ) -> c_int;
}

#[derive(Debug)]
pub struct AccessToken(String);

impl AccessToken {
    pub async fn from_uri(uri: &str, scopes: &str) -> Result<Self> {
        let uri = CString::new(uri)?;
        let scopes = CString::new(scopes)?;

        let mut token_buffer = Vec::new();
        let mut token_len: c_int = 0;

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

        token_buffer.reserve(token_len as usize);

        let result = unsafe {
            CreateAccessToken(
                uri.as_ptr(),
                scopes.as_ptr(),
                token_buffer.as_mut_ptr(),
                &mut token_len,
            )
        };

        if result == 0 {
            return Err(anyhow!("Error calling AccessToken"));
        }

        let token_string = unsafe { CStr::from_ptr(token_buffer.as_ptr()) }.to_str()?;
        Ok(Self(token_string.to_string()))
    }

    pub async fn raw_authorization_header(&self) -> Result<(String, String)> {
        Ok(("authorization".to_owned(), format!("Bearer {}", self.0)))
    }
}
