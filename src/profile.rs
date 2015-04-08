use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::fs::File;
use tempdir::TempDir;
use std::io::Result as IoResult;
use preferences::{Preferences, PrefValue};

pub struct Profile {
    pub path: PathBuf,
    pub preferences: Preferences,
    pub temp_dir: Option<TempDir>
}

impl Profile {
    pub fn new(opt_path: Option<&Path>) -> IoResult<Profile> {
        let mut temp_dir = None;
        let path = match opt_path {
            Some(p) => p.to_path_buf(),
            None => {
                let dir = try!(TempDir::new("rust_mozprofile"));
                let temp_path = dir.path().to_path_buf();
                temp_dir = Some(dir);
                temp_path
            }
        };

        debug!("Using profile path {}", path.to_str().unwrap());

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
            try!(prefs_file.write(
                &format!("user_pref(\"{}\", {});\n", key, value_str)[..].as_bytes()));
        }

        Ok(())
    }
}
