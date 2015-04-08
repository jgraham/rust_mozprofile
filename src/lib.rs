#[macro_use]
extern crate log;

extern crate tempdir;

pub mod profile;
pub mod preferences;

#[cfg(test)]
mod test {
    use std::fs::File;
    use profile::Profile;
    use preferences::PrefValue;

    #[test]
    fn it_works() {
        let mut prof = Profile::new(None).unwrap();

        prof.preferences.values.insert("app.update.enabled".to_string(),
                                       PrefValue::PrefBool(false));
        prof.preferences.values.insert("extensions.enabledScopes".to_string(),
                                       PrefValue::PrefString("foo".to_string()));
        prof.write_prefs().unwrap();
        let mut user_file = File::open(&prof.path.with_filename("user.js"));
        println!("{}", user_file.read_to_string().unwrap());
    }
}
