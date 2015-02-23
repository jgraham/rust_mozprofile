use std::old_path::Path;
use std::old_io::{File, IoResult, TempDir};

use preferences::{Preferences, PrefValue};

pub struct Profile {
    pub path: Path,
    pub preferences: Preferences,
    pub temp_dir: Option<TempDir>
}

impl Profile {
    pub fn new(opt_path: Option<Path>) -> IoResult<Profile> {
        let mut temp_dir = None;
        let path = match opt_path {
            Some(p) => p,
            None => {
                temp_dir = Some(try!(TempDir::new("rust_mozprofile")));
                temp_dir.as_ref().unwrap().path().clone()
            }
        };

        debug!("Using profile path {}", path.as_str().unwrap());

        Ok(Profile {
            path: path,
            preferences: Preferences::new(),
            temp_dir: temp_dir
        })
    }

    pub fn write_prefs(&self) -> IoResult<()> {
        let prefs_path = self.path.join("user.js");

        let mut prefs_file = try!(File::create(&prefs_path));

        for (key, value) in self.preferences.iter() {
            let value_str = match *value {
                PrefValue::PrefBool(true) => "true".to_string(),
                PrefValue::PrefBool(false) => "false".to_string(),
                PrefValue::PrefString(ref x) => format!("\"{}\"", x),
                PrefValue::PrefInt(ref x) => format!("{}", x)
            };
            try!(prefs_file.write_str(&format!("user_pref(\"{}\", {});\n", key, value_str)[..]));
        }

        Ok(())
    }
}
