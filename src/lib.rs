pub mod controller;
pub mod db;
pub mod log;
pub mod res;
pub mod util;
pub mod win;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    lazy_static::initialize(&crate::controller::base::INIT);

    let mut builder = tauri::Builder::default();

    builder = builder
        .setup(|app| {
            util::env::set_app_handle(app.handle().clone());
            log::init_log4rs();
            db::load_config();
            win::setup_window(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_http::init());

    builder
        .register_uri_scheme_protocol("keeyuu", move |_ctx, request| {
            res::get_http_response(request)
        })
        .invoke_handler(tauri::generate_handler![
            controller::get_command_registry,
            controller::example_command,
            controller::invoke_dynamic_command,
            db::sqlite_exec,
            db::sqlite_query,
            log::log4rs_info,
            log::log4rs_warn,
            res::ui_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
