use std::time::Duration;
use tauri_plugin_http::reqwest;
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
