# Hyprspace

## About this Repo:

This little Application is something I cooked up while on
vacation.

Since I have no idea on how to write proper Rust code (or JavaScript for that matter), you 
can expect this Application to contain some of the most disgusting code you've ever seen.

<img src="./the_dev.jpg" width=200, height=200/>

*pictured: the developer that built this app*

In the following README I'll try to document this abomination as best as
I can for any unfortunate soul trying to work with this piece of junk.

Since I'm not the best at documenting my code, I'll ask you to bear with me as I try to explain the basics of this code.

*I apologise for any spelling mistakes beforehand, english is not my first
language.*

## The Basics:

The Application will be started once and then simply remain in the Background until called again with a shortcut
defined in the WM - Settings

## Configuration and ✨Styling✨:


### Config file location:
`$HOME/.config/hypspace/hyprspace.toml` -> usually translates to `/home/yourusernamehere/.config/hyprspace/hyprspace.toml`

### Styling file location:
`$HOME/.config/hyprspace/style.css` -> usually translates to `/home/yourusernamehere/.config/hyprspace/style.css`

### Config file:

`Hyprspace.toml` -> The main configuration for the Application

### Hints:

It is important, that Hyprspace is started **AFTER** any modifications to the `$PATH` variable, otherwise it will
not be able to find and watch certain directories.
    
### Styling:

`style.css` -> The styling file for the Application

Styling is done via plain old CSS.

To get an overview of the html structure simply right click and inspect the window. (like a website, i mean it is simply a webview)

## Used Crates:

```toml
[build-dependencies]
tauri-build = { version = "1.2", features = [] }

[dependencies]
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
tauri = { version = "1.2", features = ["cli", "shell-open", "window-center", "window-close", "window-hide"] }
log = "0.4.17"
env_logger ="0.10.0"
notify = "5.0.0"
directories = "4.0.1"
sysinfo ="0.27.7"
```

## Debugging and Logs:

This application uses the [`env_logger`](https://crates.io/crates/env_logger) crate for logging messages to `stdout` and `stderr`

To show messages while running the executable the `RUST_LOG` environment
variable must be set.

**For log-level `WARN`:**
```
RUST_LOG=WARN
./hyprspace
```

**For log-level `INFO`:**
```
RUST_LOG=INFO
./hyprspace
```

**For log-level `DEBUG`:**
```
RUST_LOG=DEBUG
./hyprspace
```

**For log-level `TRACE`:**
```
RUST_LOG=TRACE
./hyprspace
```

## Used Functions:

### get_search_results: get raw search result HTML

Returns a String containing raw HTML containing all found Application items.

[SOURCE](./src-tauri/src/events/functions.rs)

## Possible Events:

### ErrorEvent: 'error-message'

Emitted by the frontend, if an error occurs there.

```json
{
    error: "ErrorMessage",
    message: "An Error occurred in component: ComponentName ==> Check the Logs. Error was: ErrorMessage",
    component_of_origin: "ComponentName"
}
```


### SearchInputEvent: 'search-message'

Emitted on input in search bar (unused for now)

```json
{
    message: "The UserInput",
    component_of_origin: "ComponentName"
}
```

### LoadStyleSheetMessage: 'style-message'

Emitted on startup to tell the application to load
the stylesheet from .config directory

*(Note: if no stylesheet is present, an empty string will be sent)*

```json
{
    message: "/the/path/to/the/style.css"
}
```