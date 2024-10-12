mod structs;
mod util;
mod xzmu;
use tauri::Manager;
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
            .on_window_event(|window, event| match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    window.hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            })
            .plugin(tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                Some(vec!["--flag1", "--flag2"]), // 向应用传递的参数
            ))
            .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }))
            .setup(|app| {
                let quit_i = MenuItem::with_id(app, "quit", "关闭程序", true, None::<&str>)?;
                let show_i = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

                let _tray = TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .menu(&menu)
                    .menu_on_left_click(true)
                    .on_menu_event(|app, event| match event.id.as_ref() {
                        "quit" => {
                            println!("quit");
                            app.exit(0);
                        }
                        "show" => {
                            println!("show");
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
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
