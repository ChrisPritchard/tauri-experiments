# Absolutely basic example

Created by:

1. creating a ui folder with a index.html page
2. running `cargo tauri init`, with no build commands and the paths set to `../ui`

Thats all thats required to get it working. `cargo tauri dev` creates a watcher like run command

## Adding callable functions

To add a server function that can be called, four more steps:

1. create a function with the `#[tauri::command]` trait:

    ```rust
    #[tauri::command]
    fn get_info() -> String {
        let args: Vec<_> = env::args().collect();
        format!("{:?}", args)
    }
    ```

2. add it to the tauri builder object: `.invoke_handler(tauri::generate_handler![get_info])`
3. add a global tauri object in the build step of tauri.conf.json: `"withGlobalTauri": true`
4. add a global invoker object in javascript: `const { invoke } = window.__TAURI__.tauri;`

The function can then be invoked like:

```javascript
    document.getElementById("get_info").addEventListener("click", () => {
        invoke('get_info', {}).then(response => {
            document.getElementById("result").innerText = response;
        });
    });
```

## Adding server state

Server state isn't tooooo tricky.

1. For readonly, add a line to builder like `.manage(GameState { counter: 0 })`
2. then access from functions with `fn read_count(state: State<GameState>)`

But this will be readonly. For mutable, use std::sync::Mutex:

3. `.manage(Mutex::new(GameState { counter: 0 }))`
4. And `state: State<Mutex<GameState>>`
5. It can be altered like: `(*state.lock().unwrap()).counter += 1;``

## Events

Events can be sent back and forth as needed. On the server, adding `window: Window` to functions allows them to 'emit' events.

On the client, importing listen like `const { listen } = window.__TAURI__.event;` allows you to listen for events and execute actions: `listen('counter_update', evt => { })`