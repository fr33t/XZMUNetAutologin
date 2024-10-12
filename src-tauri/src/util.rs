use std::{path::PathBuf, time::Duration};
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_http::reqwest;

pub(crate) async fn test_xzmu_connection() -> bool {
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
