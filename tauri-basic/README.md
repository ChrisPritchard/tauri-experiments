# Absolutely basic example

Created by:

1. creating a ui folder with a index.html page
2. running `cargo tauri init`, with no build commands and the paths set to `../ui`

Thats all thats required to get it working. `cargo tauri dev` creates a watcher like run command

To add a server function that can be called, four more steps:

3. create a function with the `#[tauri::command]` trait:

    ```rust
    #[tauri::command]
    fn get_info() -> String {
        let args: Vec<_> = env::args().collect();
        format!("{:?}", args)
    }
    ```

4. add it to the tauri builder object: `.invoke_handler(tauri::generate_handler![get_info])`
5. add a global tauri object in the build step of tauri.conf.json: `"withGlobalTauri": true`
6. add a global invoker object in javascript: `const { invoke } = window.__TAURI__.tauri;`

The function can then be invoked like:

```javascript
    document.getElementById("get_info").addEventListener("click", () => {
        invoke('get_info', {}).then(response => {
            document.getElementById("result").innerText = response;
        });
    });
```