use std::error::Error;
use tauri::{App, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::util::env;
pub fn setup_window(app: &App) -> Result<(), Box<dyn Error>> {
    let init_script = format!(
        r#"
        window.__LY_SDK__ = {{ platform: '{}', arch: '{}', version: '{}' }};
    "#,
        env::get_plaform(),
        env::get_arch(),
        env::get_version()
    );

    let mut win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .initialization_script(&init_script);

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        win_builder = win_builder
        .inner_size(800.0, 600.0)
        .title("")
        .devtools(true);
    }

    //.decorations(false);

    #[cfg(target_os = "macos")]
    {

        use cocoa::appkit::{NSColor, NSWindow};
        use cocoa::base::YES;
        use cocoa::base::{id, nil};
        let window = win_builder.build().unwrap(); 
        //window.open_devtools();
        // #[cfg(debug_assertions)] // only include this code on debug builds
        // { 
        //   window.open_devtools();
        //   window.close_devtools();
        // }
        //let window = app.get_webview_window("main").unwrap();
        let ns_window = window.ns_window().unwrap() as id;
        unsafe {
            ns_window.setTitlebarAppearsTransparent_(YES);
            let bg_color = NSColor::colorWithRed_green_blue_alpha_(
                nil,
                0.0 / 255.0,
                0.0 / 255.0,
                0.0 / 255.0,
                0.0,
            );
            ns_window.setBackgroundColor_(bg_color);
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        let window = win_builder.build().unwrap();
    }

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
