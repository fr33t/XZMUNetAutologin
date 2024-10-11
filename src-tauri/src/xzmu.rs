use std::fs::File;
use std::io::BufReader;

use crate::structs::*;
use crate::util;
use tauri::Runtime;

#[tauri::command]
pub async fn test_network() -> i32 {
    let xzmu_result = util::test_command_xzmu().await;
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
    Ok(util::get_xzmu_config_path(&app)
        .await
        .to_string_lossy()
        .to_string())
}

#[tauri::command]
pub async fn get_account<R: Runtime>(app: tauri::AppHandle<R>) -> Option<XZMUAccount> {
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
    let config_path = util::get_xzmu_config_path(&app).await;
    let file_writer = File::create(config_path).unwrap();
    serde_json::to_writer(file_writer, &account).unwrap();
    true
}
