use std::error::Error;
use tauri::{App, Manager,WebviewUrl,WebviewWindowBuilder};

use crate::util::env;
pub fn setup_window(app: &App) -> Result<(), Box<dyn Error>> {
    let is_mobile = {
        #[cfg(any(target_os = "ios", target_os = "android"))]
        {
            true
        }
        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        {
            false
        }
    };

    let init_script = format!(
        r#"
        window.__LY_SDK__ = {{ platform: '{}', arch: '{}', version: '{}',is_mobile: {} }};
    "#,
        env::get_plaform(),
        env::get_arch(),
        env::get_version(),
        is_mobile
    );
    let _win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
          .initialization_script(&init_script)
          .build()?;
        
    // for (_label, window) in app.webview_windows() { 
    //     let _ = window.eval(&init_script);
    // }

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

        use tauri::{Size, PhysicalSize};
        let window = app.get_webview_window("main").unwrap();
        let ns_window = window.ns_window().unwrap() as id;
        let _ = window.set_size(Size::Physical(PhysicalSize {
            width: 800,
            height: 600,
        }));
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
    }

    Ok(())
}
