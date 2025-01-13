~~~
      {
        "title": "iGPT",
        "width": 800,
        "height": 600,
        "resizable": true,
        "fullscreen": false,
        "decorations": false
      }
~~~

~~~
      cargo tauri icon ./icons/icon_512x512_w.png

      cargo tauri build
~~~

// let mut resp = ResponseBuilder::new()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .header(CONTENT_TYPE, "text/plain")
//                 .body("e.to_string()".as_bytes().to_vec()).unwrap();