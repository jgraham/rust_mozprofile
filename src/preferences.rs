use std::collections::HashMap;
use std::collections::hash_map::Iter;

#[derive(Clone)]
pub enum PrefValue {
    PrefBool(bool),
    PrefString(String),
    PrefInt(isize)
}

pub struct Preferences {
    pub values: HashMap<String, PrefValue>
}

impl Preferences {
    pub fn new() -> Preferences {
        Preferences {
            values: HashMap::new()
        }
    }

    pub fn insert(&mut self, name: &str, value: PrefValue) {
        self.values.insert(name.to_string(), value);
    }

    pub fn insert_vec(&mut self, preferences: &[(&str, PrefValue)]) {
        for &(name, ref value) in preferences.iter() {
            self.insert(name, (*value).clone());
        }
    }

    pub fn iter(&self) -> Iter<String, PrefValue> {
        self.values.iter()
    }
}

pub static FIREFOX_PREFERENCES: [(&'static str, PrefValue); 17] = [
    // Don't automatically update the application
    ("app.update.enabled", PrefValue::PrefBool(false)),
    // Don't restore the last open set of tabs if the browser has crashed
    ("browser.sessionstore.resume_from_crash", PrefValue::PrefBool(false)),
    // Don't check for the default web browser during startup
    ("browser.shell.checkDefaultBrowser", PrefValue::PrefBool(false)),
    // Don't warn on exit when multiple tabs are open
    ("browser.tabs.warnOnClose", PrefValue::PrefBool(false)),
    // Don't warn when exiting the browser
    ("browser.warnOnQuit", PrefValue::PrefBool(false)),
    // Don't send Firefox health reports to the production server
    //"datareporting.healthreport.documentServerURI", "http://%(server)s/healthreport/",
    // Only install add-ons from the profile and the application scope
    // Also ensure that those are not getting disabled.
    // see: https://developer.mozilla.org/en/Installing_extensions
    ("extensions.enabledScopes", PrefValue::PrefInt(5)),
    ("extensions.autoDisableScopes", PrefValue::PrefInt(10)),
    // Don't send the list of installed addons to AMO
    ("extensions.getAddons.cache.enabled", PrefValue::PrefBool(false)),
    // Don't install distribution add-ons from the app folder
    ("extensions.installDistroAddons", PrefValue::PrefBool(false)),
    // Dont' run the add-on compatibility check during start-up
    ("extensions.showMismatchUI", PrefValue::PrefBool(false)),
    // Don't automatically update add-ons
    ("extensions.update.enabled", PrefValue::PrefBool(false)),
    // Don't open a dialog to show available add-on updates
    ("extensions.update.notifyUser", PrefValue::PrefBool(false)),
    // Enable test mode to run multiple tests in parallel
    ("focusmanager.testmode", PrefValue::PrefBool(true)),
    // Enable test mode to not raise an OS level dialog for location sharing
    ("geo.provider.testing", PrefValue::PrefBool(true)),
    // Suppress delay for main action in popup notifications
    ("security.notification_enable_delay", PrefValue::PrefInt(0)),
    // Suppress automatic safe mode after crashes
    ("toolkit.startup.max_resumed_crashes", PrefValue::PrefInt(-1)),
    // Don't report telemetry information
    ("toolkit.telemetry.enabled", PrefValue::PrefBool(false)),
];
