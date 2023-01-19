/*
The Freedesktop spec:
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

*/