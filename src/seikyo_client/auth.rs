use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use chrono;
use chrono::TimeZone;
use reqwest;
use serde::Deserialize;

const SIGN_IN_ENDPOINT: &str = "https://mb.seikyou.jp/mobileapp_common/tohoku/getToken2.do";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AccessToken {
    token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthData {
    app_id: Option<String>,
    access_token: Option<AccessToken>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ErrorDetail {
    error_title: String,
    error_message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignInResult {
    status: String,
    data: Option<AuthData>,
    status_message: String,
    klas_error_detail: Option<ErrorDetail>,
}

#[derive(Debug)]
struct ApiError {
    error_msg: Option<String>,

}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.error_msg {
            None => write!(f, "Unknown error"),
            Some(msg) => write!(f, "API Error: {}", msg)
        }
    }
}

impl Error for ApiError {}

impl ApiError {
    pub fn from_result(result: Option<ErrorDetail>) -> ApiError {
        match result {
            None => ApiError { error_msg: None },
            Some(detail) => ApiError { error_msg: Some(detail.error_message) }
        }
    }
}

pub fn get_token(id: String, pass: String) -> Result<String, Box<dyn std::error::Error>> {
    let jst_datetime = chrono::FixedOffset::east(9 * 3600).from_utc_datetime(&chrono::Utc::now().naive_utc());
    let datetime_param = jst_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    let mut password_hash_original = id.to_owned();
    password_hash_original.push_str(&pass);
    let digest = md5::compute(password_hash_original);
    let digest_vec = digest.to_vec();
    let digest_hex = hex::encode(digest_vec);

    let mut params = HashMap::new();
    params.insert("appVersion", "5.0.0");
    params.insert("mpLoginId", id.as_str());
    params.insert("mpPassword", digest_hex.as_str());
    params.insert("nativeAppId", "6651000001");
    params.insert("userAgent", "iOS 15.0.2");
    params.insert("userDatetime", datetime_param.as_str());

    let client = reqwest::blocking::Client::new();
    let response = client.post(SIGN_IN_ENDPOINT)
        .form(&params)
        .send()?
        .text()?;

    let result: SignInResult = serde_json::from_str(response.as_str())?;
    if result.status != "0" {
        return Err(Box::new(ApiError::from_result(result.klas_error_detail)));
    }
    match result.data {
        None => Err(Box::new(ApiError::from_result(result.klas_error_detail))),
        Some(d) => match d.access_token {
            None => Err(Box::new(ApiError::from_result(result.klas_error_detail))),
            Some(token) => Ok(token.token)
        }
    }
}
