use std::collections::HashMap;

use chrono;
use chrono::TimeZone;
use reqwest;

const SIGN_IN_ENDPOINT: &str = "https://mb.seikyou.jp/mobileapp_common/tohoku/getToken2.do";

pub fn get_token(id: String, pass: String) -> Result<String, reqwest::Error> {
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

    println!("{:?}", response);

    return Result::Ok(String::from(""));
}
