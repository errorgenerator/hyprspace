# Hyprspace

**HYPRSPACE IS MAINLY FOR MY PERSONAL USE, SO PLEASE BE PATIENT. IT WAS NEVER INTENDED TO BE MORE THAN JUST A SIDE PROJECT.
THUS, YOU CAN EXPECT TO FIND A LOT OF BAD CODE HERE AS WELL AS ONLY LACKLUSTER DOCUMENTATION**

*This app has mainly been used by me to get a "feel" for Tauri for an upcoming project called
shitb0x-radio.*

*i will add more documentation soon, i promise ...*

- [Hyprspace](#hyprspace)
  * [β About this Repo:](#--about-this-repo-)
  * [ποΈ The Basics:](#----the-basics-)
  * [π οΈ Configuration and β¨Stylingβ¨:](#----configuration-and--styling--)
    + [βοΈ Config file location:](#---config-file-location-)
    + [β¨ Styling file location:](#--styling-file-location-)
    + [βοΈ Config file:](#---config-file-)
    + [β¨ Styling:](#--styling-)
    + [β How do I change the look of the Application-Icons?](#--how-do-i-change-the-look-of-the-application-icons-)
    + [β Hints:](#--hints-)
  * [π¦ Used Crates:](#---used-crates-)
  * [π§βπ» Debugging and Logs:](#------debugging-and-logs-)
  * [π» Used Functions:](#---used-functions-)
    + [get_search_results: get raw search result HTML](#get-search-results--get-raw-search-result-html)
  * [βοΈ Events:](#---events-)
    + [ErrorEvent: 'error-message'](#errorevent---error-message-)
    + [SearchInputEvent: 'search-message'](#searchinputevent---search-message-)
    + [LoadStyleSheetMessage: 'style-message'](#loadstylesheetmessage---style-message-)

## β About this Repo:

This little Application is something I cooked up while on
vacation.

Since I have no idea on how to write proper Rust code (or JavaScript for that matter), you 
can expect this Application to contain some of the most disgusting code you've ever seen.

<img src="./the_dev.jpg" width=200/>

*pictured: the developer that built this app*

In the following README I'll try to document this abomination as best as
I can for any unfortunate soul trying to work with this piece of junk.

Since I'm not the best at documenting my code, I'll ask you to bear with me as I try to explain the basics of this code.

*I apologise for any spelling mistakes beforehand, english is not my first
language.*

## ποΈ The Basics:

The Application will be started once and then simply remain in the Background until called again with a shortcut
defined in the WM - Settings

## π οΈ Configuration and β¨Stylingβ¨:


### βοΈ Config file location:
`$HOME/.config/hypspace/hyprspace.toml` -> usually translates to `/home/yourusernamehere/.config/hyprspace/hyprspace.toml`

### β¨ Styling file location:
`$HOME/.config/hyprspace/hypr.css` -> usually translates to `/home/yourusernamehere/.config/hyprspace/hypr.css`

### βοΈ Config file:

`Hyprspace.toml` -> The main configuration for the Application


    
### β¨ Styling:

`hypr.css` -> The styling file for the Application

Styling is done via plain old CSS.

To get an overview of the html structure simply right click and inspect the window. (like a website, i mean it is simply a webview)


### β How do I change the look of the Application-Icons?

Hyprspace simply follows the current icon theme.

It will try to determine the used icon-theme in the 
following order:

1. Icon-Theme used by **GTK3**
2. Icon-Theme used by **GTK2**
3. Icon-Theme used by **Gsettings**
4. Icon-Theme used by **KDE**
5. Icon-Theme used by **ThemeConf**

It will return name of the first theme it found.

So if you have (for example) Adwaita as your GTK3 Icon-Theme and Breeze as your KDE-Plasma Icon-Theme set, it will use the Adwaita theme, since it first checks the GTK3 theme before KDE.

Note: if no Theme can be determined it will use the `hicolor` theme as fallback!

My Advice: Use something like **[lxappearance](https://github.com/lxde/lxappearance)** to change your icon theme and then the icons will change after a restart of Hyprspace.

### β Hints:

It is important, that Hyprspace is started **AFTER** any modifications to the `$PATH` variable, otherwise it will
not be able to find and watch certain directories.

**Keep in mind, that hyprspace will not show ANY UI on startup.
Instead, it listens on a socket (/tmp/hyprspace.sock by default) for a command
to show itself. send the string "TOGGLE" over the socket to toggle visibility,
send the string "SHOW" for the app to show itself. send the string "HIDE" for the app
to hide again.**

*this is probably important enough to get its own section, but for now, we really don't need to document anything else, since this app is not intended for widespread use.*

## π¦ Used Crates:

```toml
[build-dependencies]
tauri-build = { version = "1.2", features = [] }

[dependencies]
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
tauri = { version = "1.2", features = ["cli", "clipboard", "fs-copy-file", "fs-exists", "fs-read-dir", "fs-read-file", "path-all", "protocol-all", "reqwest-client", "shell-execute", "shell-open", "window-center", "window-close", "window-hide", "wry"] }
log = "0.4.17"
env_logger = "0.10.0"
notify = "5.0.0"
notify-debouncer-mini = "0.2.1"
directories = "4.0.1"
sysinfo = "0.27.7"
freedesktop-icons = "0.2.3"
linicon-theme = "1.2.0"
cached = "0.42.0"
rust-fuzzy-search = "0.1.1"
toml = "0.7.0"
```

## π§βπ» Debugging and Logs:

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

## π» Used Functions:

### get_search_results: get raw search result HTML

Returns a String containing raw HTML containing all found Application items.

[SOURCE](./src-tauri/src/events/functions.rs)

## βοΈ Events:

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