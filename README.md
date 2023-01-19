# Hyprspace

## ❓ About this Repo:

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

## 🗒️ The Basics:

The Application will be started once and then simply remain in the Background until called again with a shortcut
defined in the WM - Settings

## 🛠️ Configuration and ✨Styling✨:


### ⚙️ Config file location:
`$HOME/.config/hypspace/hyprspace.toml` -> usually translates to `/home/yourusernamehere/.config/hyprspace/hyprspace.toml`

### ✨ Styling file location:
`$HOME/.config/hyprspace/hypr.css` -> usually translates to `/home/yourusernamehere/.config/hyprspace/hypr.css`

### ⚙️ Config file:

`Hyprspace.toml` -> The main configuration for the Application


    
### ✨ Styling:

`hypr.css` -> The styling file for the Application

Styling is done via plain old CSS.

To get an overview of the html structure simply right click and inspect the window. (like a website, i mean it is simply a webview)


### ❓ How do I change the look of the Icons?

```
tldr;
it simply follows the current icon theme. use something like lxappearance to change them.
```

This application follows the specification of the Icon-Lookup algorithm from [Freedesktop](https://specifications.freedesktop.org).


more specifically [THIS ONE](https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html#icon_lookup)

<details>
<summary>The Lookup Algorithm Specifikation</summary>

```
Icon Lookup

The icon lookup mechanism has two global settings, the list of base directories and the internal name of the current theme. Given these we need to specify how to look up an icon file from the icon name, the nominal size and the scale.

The lookup is done first in the current theme, and then recursively in each of the current theme's parents, and finally in the default theme called "hicolor" (implementations may add more default themes before "hicolor", but "hicolor" must be last). As soon as there is an icon of any size that matches in a theme, the search is stopped. Even if there may be an icon with a size closer to the correct one in an inherited theme, we don't want to use it. Doing so may generate an inconsistant change in an icon when you change icon sizes (e.g. zoom in).

The lookup inside a theme is done in three phases. First all the directories are scanned for an exact match, e.g. one where the allowed size of the icon files match what was looked up. Then all the directories are scanned for any icon that matches the name. If that fails we finally fall back on unthemed icons. If we fail to find any icon at all it is up to the application to pick a good fallback, as the correct choice depends on the context.

The exact algorithm (in pseudocode) for looking up an icon in a theme (if the implementation supports SVG) is:

FindIcon(icon, size, scale) {
  filename = FindIconHelper(icon, size, scale, user selected theme);
  if filename != none
    return filename

  filename = FindIconHelper(icon, size, scale, "hicolor");
  if filename != none
    return filename

  return LookupFallbackIcon (icon)
}
FindIconHelper(icon, size, scale, theme) {
  filename = LookupIcon (icon, size, scale, theme)
  if filename != none
    return filename

  if theme has parents
    parents = theme.parents

  for parent in parents {
    filename = FindIconHelper (icon, size, scale, parent)
    if filename != none
      return filename
  }
  return none
}
     

With the following helper functions:

LookupIcon (iconname, size, scale, theme) {
  for each subdir in $(theme subdir list) {
    for each directory in $(basename list) {
      for extension in ("png", "svg", "xpm") {
        if DirectoryMatchesSize(subdir, size, scale) {
          filename = directory/$(themename)/subdir/iconname.extension
          if exist filename
	    return filename
        }
      }
    }
  }
  minimal_size = MAXINT
  for each subdir in $(theme subdir list) {
    for each directory in $(basename list) {
      for extension in ("png", "svg", "xpm") {
        filename = directory/$(themename)/subdir/iconname.extension
        if exist filename and DirectorySizeDistance(subdir, size, scale) < minimal_size {
	   closest_filename = filename
	   minimal_size = DirectorySizeDistance(subdir, size, scale)
        }
      }
    }
  }
  if closest_filename set
     return closest_filename
  return none
}

LookupFallbackIcon (iconname) {
  for each directory in $(basename list) {
    for extension in ("png", "svg", "xpm") {
      if exists directory/iconname.extension
        return directory/iconname.extension
    }
  }
  return none
}

DirectoryMatchesSize(subdir, iconsize, iconscale) {
  read Type and size data from subdir
  if Scale != iconscale
     return False;
  if Type is Fixed
    return Size == iconsize
  if Type is Scaled
    return MinSize <= iconsize <= MaxSize
  if Type is Threshold
    return Size - Threshold <= iconsize <= Size + Threshold
}

DirectorySizeDistance(subdir, iconsize, iconscale) {
  read Type and size data from subdir
  if Type is Fixed
    return abs(Size*Scale - iconsize*iconscale)
  if Type is Scaled
    if iconsize*iconscale < MinSize*Scale
        return MinSize*Scale - iconsize*iconscale
    if iconsize*iconscale > MaxSize*Scale
        return iconsize*iconscale - MaxSize*Scale
    return 0
  if Type is Threshold
    if iconsize*iconscale < (Size - Threshold)*Scale
        return MinSize*Scale - iconsize*iconscale
    if iconsize*iconsize > (Size + Threshold)*Scale
        return iconsize*iconsize - MaxSize*Scale
    return 0
}

In some cases you don't always want to fall back to an icon in an inherited theme. For instance, sometimes you look for a set of icons, prefering any of them before using an icon from an inherited theme. To support such operations implementations can contain a function that finds the first of a list of icon names in the inheritance hierarchy. I.E. It would look something like this:

FindBestIcon(iconList, size, scale) {
  filename = FindBestIconHelper(iconList, size, scale, user selected theme);
  if filename != none
    return filename

  filename = FindBestIconHelper(iconList, size, scale, "hicolor");
  if filename != none
    return filename

  for icon in iconList {
    filename = LookupFallbackIcon (icon)
    if filename != none
      return filename
  }
  return none;
}
FindBestIconHelper(iconList, size, scale, theme) {
  for icon in iconList {
    filename = LookupIcon (icon, size, theme)
    if filename != none
      return filename
  }

  if theme has parents
    parents = theme.parents

  for parent in parents {
    filename = FindBestIconHelper (iconList, size, scale, parent)
    if filename != none
      return filename
  }
  return none
}

This can be very useful for example when handling mimetype icons, where there are more and less "specific" versions of icons.


```

</details>

### ❗ Hints:

It is important, that Hyprspace is started **AFTER** any modifications to the `$PATH` variable, otherwise it will
not be able to find and watch certain directories.

## 📦 Used Crates:

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

## 🧑‍💻 Debugging and Logs:

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

## 💻 Used Functions:

### get_search_results: get raw search result HTML

Returns a String containing raw HTML containing all found Application items.

[SOURCE](./src-tauri/src/events/functions.rs)

## ✉️ Events:

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