~~~
      {
        "title": "iGPT",
        "width": 800,
        "height": 600,
        "resizable": true,
        "fullscreen": false,
        "decorations": false
      }
   // {
      //   "title": "",
      //   "width": 800,
      //   "height": 600,
      //   "resizable": true,
      //   "fullscreen": false,
      //   "decorations": false,
      //   "transparent": true,
      //   "devtools": true
      // }

          // "clipboard-manager:allow-write-text", 
    // "clipboard-manager:allow-write-html",
    // "clipboard-manager:allow-write-image",
    // "clipboard-manager:allow-clear",
    // "clipboard-manager:allow-read-image",
    // "clipboard-manager:allow-read-text",
~~~

~~~
      cargo tauri icon --ios-color=transparent ./icons/luotuo.png

      cargo tauri build


      cargo tauri [android|ios] init 
      cargo tauri android init

      cargo tauri [android|ios] dev

      cargo tauri android dev "my_avd"
 

      export OPENSSL_DIR=/usr/local/opt/openssl
      cargo tauri android build --apk

      sdkmanager "system-images;android-33;default;x86_64"
      sdkmanager "platforms;android-33" "platform-tools" "ndk;25.0.8775105" "build-tools;33.0.0"
~~~
查看可用镜像
sdkmanager --list
下载镜像
sdkmanager "platform-tools" "platforms;android-29" "emulator" "system-images;android-29;default;x86_64"
安装模拟器
avdmanager delete avd -n my_avd
avdmanager create avd -n my_avd -k "system-images;android-33;default;x86_64"
启动模拟器
emulator -avd my_avd

// let mut resp = ResponseBuilder::new()
//                 .status(StatusCode::INTERNAL_SERVER_ERROR)
//                 .header(CONTENT_TYPE, "text/plain")
//                 .body("e.to_string()".as_bytes().to_vec()).unwrap();