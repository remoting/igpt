use cocoa::foundation::NSRect;
use std::error::Error;
use tauri::{App, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::{util::env, win};
pub fn setup_window(app: &App) -> Result<(), Box<dyn Error>> {
    let init_script = format!(
        r#"
        window.__LY_SDK__ = {{ platform: '{}', arch: '{}', version: '{}' }};
    "#,
        env::get_plaform(),
        env::get_arch(),
        env::get_version()
    );
    for (_label, window) in app.webview_windows() {
        //println!("Window label: {}", label);
        let _ = window.eval(&init_script);
    }

    // let mut win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
    //     .initialization_script(&init_script);

    // #[cfg(not(any(target_os = "android", target_os = "ios")))]
    // {
    //     win_builder = win_builder
    //     .inner_size(800.0, 600.0)
    //     .title("")
    //     .devtools(true);
    // }

    //.decorations(false);

    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::{
            NSColor, NSView, NSWindow, NSWindowStyleMask, NSWindowTitleVisibility,
        }; 
        use cocoa::base::{id, nil, YES};  
        use objc::{msg_send, sel, sel_impl};

        let window = app.get_webview_window("main").unwrap();
        let ns_window = window.ns_window().unwrap() as id;

        unsafe {
            // 设置窗口样式掩码
            let style_mask = NSWindowStyleMask::NSTitledWindowMask
                | NSWindowStyleMask::NSClosableWindowMask
                | NSWindowStyleMask::NSMiniaturizableWindowMask
                | NSWindowStyleMask::NSResizableWindowMask
                | NSWindowStyleMask::NSFullSizeContentViewWindowMask;
            let _: () = msg_send![ns_window, setStyleMask: style_mask];
            // 设置标题栏透明
            let _: () = msg_send![ns_window, setTitlebarAppearsTransparent: YES];
            // 隐藏标题
            let _: () = msg_send![ns_window, setTitleVisibility: NSWindowTitleVisibility::NSWindowTitleHidden];
            // 设置窗口背景颜色为透明
            let _: () = msg_send![ns_window, setBackgroundColor: NSColor::clearColor(nil)];
            // // 强制更新窗口布局
            // let content_view: id = msg_send![ns_window, contentView];
            // let _: () = msg_send![ns_window, setContentView: content_view];
        }
        //let _ = window.set_decorations(true);
    }
    //window.open_devtools();
    // #[cfg(debug_assertions)] // only include this code on debug builds
    // {
    //   window.open_devtools();
    //   window.close_devtools();
    // }
    //let window = app.get_webview_window("main").unwrap();
    // window.set_title_bar_style(TitleBarStyle::transient);
    //window.set_title("")?;
    // #[cfg(not(target_os = "macos"))]
    // {
    //     let window = win_builder.build().unwrap();
    // }

    // // set background color only when building for macOS
    // #[cfg(target_os = "macos")]
    // {
    //     use cocoa::appkit::{NSColor, NSWindow};
    //     use cocoa::base::{id, nil};

    //     let ns_window = window.ns_window().unwrap() as id;
    //     unsafe {
    //         let bg_color = NSColor::colorWithRed_green_blue_alpha_(
    //             nil,
    //             50.0 / 255.0,
    //             158.0 / 255.0,
    //             163.5 / 255.0,
    //             1.0,
    //         );
    //         ns_window.setBackgroundColor_(bg_color);
    //     }
    // }

    Ok(())
}
