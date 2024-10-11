mod structs;
mod util;
mod xzmu;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_autostart::MacosLauncher; // 仅在非 Android 和 iOS 上导入 MacosLauncher

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut app_builder = tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            xzmu::test_network,
            xzmu::get_conf,
            xzmu::get_account,
            xzmu::save_account,
            xzmu::login,
            xzmu::is_android,
        ]);
    // 根据平台条件性添加自启动插件

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        app_builder = app_builder.plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]), // 向应用传递的参数
        ));
    }

    app_builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
