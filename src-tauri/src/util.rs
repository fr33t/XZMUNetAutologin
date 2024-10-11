use crate::structs::XZMUNetConfig;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashMap;
use std::{path::PathBuf, time::Duration};
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_http::reqwest::{self, header};
use url::Url;

pub(crate) async fn test_command_xzmu() -> bool {
    let test_url = "http://120.95.80.23:8080/Self/login/";
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(1000)) // 设置超时为 1.5 秒
        .build();

    match client {
        Ok(client) => match client.get(test_url).send().await {
            Ok(res) => {
                println!("{:?}", res);
                true
            }
            Err(_) => false,
        },
        Err(_) => false,
    }
}

pub(crate) async fn test_internet_connection() -> bool {
    let test_url = "http://www.163.com/";
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(1000))
        .build();

    match client {
        Ok(client) => match client.get(test_url).send().await {
            Ok(res) => {
                let body = res.text().await.unwrap();
                if body.contains("http://10.1.0.212") {
                    return false;
                }
                true
            }
            Err(_) => false,
        },
        Err(_) => false,
    }
}

pub(crate) async fn get_xzmu_config_path<R: Runtime>(app: &AppHandle<R>) -> PathBuf {
    app.path()
        .app_data_dir()
        .unwrap()
        .join("xzmu-autologin.json")
}

pub(crate) async fn get_xzmu_net_config() -> XZMUNetConfig {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "User-Agent",
        header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36"),
    );

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    let response = client.get("http://10.10.0.163/").send().await.unwrap();
    let body = response.text().await.map_err(|e| e.to_string()).unwrap();
    let re = Regex::new(r#"location\.href="(.*?)""#).unwrap();
    let login_url = re.captures(&body).unwrap().get(1).unwrap().as_str();

    let s = Url::parse(login_url).unwrap();

    let a: HashMap<Cow<str>, Cow<str>> = s.query_pairs().collect();

    XZMUNetConfig {
        wlan_user_ip: a.get("wlanuserip").unwrap().to_string(),
        wlan_user_mac: a.get("wlanusermac").unwrap().to_string(),
        wlan_ac_ip: a.get("wlanacip").unwrap().to_string(),
        wlan_ac_name: a.get("wlanacname").unwrap().to_string(),
    }
}
