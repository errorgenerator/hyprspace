use freedesktop_icons::lookup;


/// Uses the freedesktop lookup algorithm to find icons for a 
/// given application name based on the current icon theme, 
/// returning the path as a string to be loaded by the frontend.
pub fn get_icon_path_for_application(icon_name: String) -> String {

  let icon_theme_name = get_icon_theme_name();

  let icon_path_buf = lookup(icon_name.as_str())
  .with_theme(icon_theme_name.as_str())
  .with_cache() // Needed bc we will do a lot of lookups
  .find();

  match icon_path_buf {
    None => {
      return String::from("");
    },
    Some(b) => {
      let path_string = String::from(b.to_string_lossy());
      return path_string;
    }
  }
}

fn get_icon_theme_name() -> String {

  let gtk3 = linicon_theme::Check::GTK3;
  let gtk2 = linicon_theme::Check::GTK2;
  let check_gsettings = linicon_theme::Check::GSettings;
  let check_kde = linicon_theme::Check::KDEGlobals;
  let themeconf = linicon_theme::Check::ThemeConf;

  let check_order = [gtk3, gtk2, check_kde, check_gsettings, themeconf];

  match linicon_theme::get_icon_theme_order(&check_order) {
    None => {
      return String::from("hicolor"); // hicolor as fallback
    },
    Some(s) => {
      return s;
    }
  }
}