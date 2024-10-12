use crate::structs::*;
use crate::util;
use log::info;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use tauri::Runtime;
use tauri_plugin_http::reqwest::{self, header};
use url::Url;

#[tauri::command]
pub async fn test_network() -> i32 {
    info!("test_network");
    let xzmu_result = util::test_xzmu_connection().await;
    let network_result = util::test_internet_connection().await;
    //

    if xzmu_result && network_result {
        // 使用校园网连接到互联网
        return 1;
    } else if xzmu_result && !network_result {
        // 已连接校园网 可能未登录
        return 2;
    } else if !xzmu_result && network_result {
        // 未连接校园网 但已连接互联网
        return 3;
    } else {
        // 未连接校园网 也未连接互联网
        return 4;
    }
}

#[tauri::command]
pub async fn get_conf<R: Runtime>(app: tauri::AppHandle<R>) -> Result<String, String> {
    info!("get_conf");
    Ok(util::get_xzmu_config_path(&app)
        .await
        .to_string_lossy()
        .to_string())
}

#[tauri::command]
pub async fn get_account<R: Runtime>(app: tauri::AppHandle<R>) -> Option<XZMUAccount> {
    info!("get_account");
    let config_path = util::get_xzmu_config_path(&app).await;
    if !config_path.exists() {
        return None;
    }
    let file_reader = BufReader::new(File::open(config_path).unwrap());
    let account: XZMUAccount = serde_json::from_reader(file_reader).unwrap();

    Some(account)
}

#[tauri::command]
pub async fn save_account<R: Runtime>(app: tauri::AppHandle<R>, account: XZMUAccount) -> bool {
    info!("save_account");
    let config_path = util::get_xzmu_config_path(&app).await;
    let file_writer = File::create(config_path).unwrap();
    serde_json::to_writer(file_writer, &account).unwrap();
    true
}

#[tauri::command]
pub async fn login(account: XZMUAccount) -> i32 {
    info!("login");
    let status = test_network().await;
    if status != 1 || status != 2 {
        return status;
    }

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "User-Agent",
        header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36"),
    );

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(8))
        .build()
        .unwrap();

    let response = match client.get("http://10.10.0.163/").send().await {
        Ok(response) => response,
        Err(_) => {
            return 1;
        }
    };

    let body = response.text().await.map_err(|e| e.to_string()).unwrap();
    let re = Regex::new(r#"location\.href="(.*?)""#).unwrap();
    let login_url = re.captures(&body).unwrap().get(1).unwrap().as_str();

    let s = Url::parse(login_url).unwrap();

    let a: HashMap<Cow<str>, Cow<str>> = s.query_pairs().collect();

    let xzmu_net_config = XZMUNetConfig {
        wlan_user_ip: a.get("wlanuserip").unwrap().to_string(),
        wlan_user_mac: a.get("wlanusermac").unwrap().to_string(),
        wlan_ac_ip: a.get("wlanacip").unwrap().to_string(),
        wlan_ac_name: a.get("wlanacname").unwrap().to_string(),
    };

    info!("{:?}", xzmu_net_config);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "User-Agent",
        header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36"),
    );
    headers.insert(
        "Referer",
        header::HeaderValue::from_static("http://10.1.0.212/"),
    );

    let login_url = format!(
        "http://10.1.0.212:801/eportal/portal/login?callback=dr1003&login_method=1&user_account=,0,{}&user_password={}&wlan_user_ip={}&wlan_user_ipv6=&wlan_user_mac={}&wlan_ac_ip={}&wlan_ac_name={}&jsVersion=4.2&terminal_type=1&lang=zh-cn&v=2833&lang=zh"
        ,account.username
        ,account.password
        ,xzmu_net_config.wlan_user_ip
        ,xzmu_net_config.wlan_user_mac.replace("-","")
        ,xzmu_net_config.wlan_ac_ip
        ,xzmu_net_config.wlan_ac_name
    );

    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    let response = match client.get(&login_url).send().await {
        Ok(response) => response,
        Err(_) => {
            return 1;
        }
    };
    println!("{:?}", login_url);
    let body = response.text().await.unwrap();
    // [Log] dr1003({"result":0,"msg":"ldap auth error","ret_code":1}); (Home.vue, line 24)
    // [Log] dr1003({"result":1,"msg":"Portal协议认证成功！"}); (Home.vue, line 24)
    println!("{:?}", body);
    if body.contains("ldap auth error") {
        return -1;
    }

    return test_network().await;
}

#[tauri::command]
pub async fn is_android() -> bool {
    let os = std::env::consts::OS.to_string();
    info!("{}", os);
    return os.contains("android");
}
