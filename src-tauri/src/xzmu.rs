use crate::util;

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
