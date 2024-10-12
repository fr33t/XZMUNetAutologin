mod structs;
mod util;
mod xzmu;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
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
        app_builder = app_builder
            .plugin(tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                Some(vec!["--flag1", "--flag2"]), // 向应用传递的参数
            ))
            .setup(|app| {
                let quit_i = MenuItem::with_id(app, "quit", "关闭程序", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&quit_i])?;

                let _tray = TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .menu_on_left_click(false)
                    .on_menu_event(|app, event| match event.id.as_ref() {
                        "quit" => {
                            println!("quit");
                            app.exit(0);
                        }
                        _ => {}
                    })
                    .build(app)?;
                Ok(())
            });
    }

    app_builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
