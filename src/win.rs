use std::error::Error;
use tauri::menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder};
use tauri::{App, TitleBarStyle, WebviewUrl, WebviewWindowBuilder};
pub fn setup_window(app: &App) -> Result<(), Box<dyn Error>> {
    let handle = app.handle();
    let toggle = MenuItemBuilder::with_id("toggle", "Toggle").build(app)?;
    let check = CheckMenuItemBuilder::new("Mark").build(app)?;
    let menu = MenuBuilder::new(handle)
        .items(&[&toggle, &check])
        //.icon("show-app", "Show App", app.default_window_icon().cloned().unwrap())
        .build()?;

    app.set_menu(menu)?;

    let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .inner_size(800.0, 600.0)
        .title("");
    //.decorations(false);

    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::{NSColor, NSWindow};
        use cocoa::base::YES;
        use cocoa::base::{id, nil};
        let window = win_builder.build().unwrap();
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
