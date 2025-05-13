use crate::config;

static HELP_GROUP: &str = "-- General --";

pub struct HelpText {
    pub name: String,
    pub group: &'static str,
}

impl HelpText {
    pub fn new(name: String, group: &'static str) -> Self {
        Self {
            name,
            group,
        }
    }
}

pub struct HelpInfo {
    pub text: HelpText,
}

impl HelpInfo {
    pub fn new(text: HelpText) -> Self {
        Self {
            text
        }
    }
}