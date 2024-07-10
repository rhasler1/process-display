use crate::config::KeyConfig;

static CMD_GROUP_GENERAL: &str = "-- General --";

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct CommandText {
    pub name: String,
    pub group: &'static str,
    pub hide_help: bool,
}

impl CommandText {
    pub const fn new(name: String, group: &'static str) -> Self {
        Self {
            name,
            group,
            hide_help: false,
        }
    }
}

pub struct CommandInfo {
    pub text: CommandText,
}

impl CommandInfo {
    pub const fn new(text: CommandText) -> Self {
        Self { text }
    }
}

pub fn move_selection(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Move selection up/down [{:?}/{:?}]",
            key.move_up, key.move_down
        ),
        CMD_GROUP_GENERAL
    )
}

pub fn selection_to_top_bottom(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Move selection to top/bottom [{:?}/{:?}]",
            key.move_top, key.move_bottom,
        ),
        CMD_GROUP_GENERAL
    )
}

pub fn filter_submit(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Filter/Submit filter [{:?}/{:?}]",
            key.filter,
            key.enter,
        ),
        CMD_GROUP_GENERAL
    )
}

pub fn change_tab(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Move tab left/right [{:?}/{:?}]",
            key.tab_left,
            key.tab_right,
        ),
        CMD_GROUP_GENERAL
    )
}

pub fn exit_popup(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Exit current screen [{:?}]",
            key.exit_popup,
        ),
        CMD_GROUP_GENERAL
    )
}

pub fn help(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Help [{:?}]",
            key.open_help,
        ),
        CMD_GROUP_GENERAL
    )
}

pub fn terminate_process(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Terminate selected process [{:?}]",
            key.terminate,
        ),
        CMD_GROUP_GENERAL
    )
}

pub fn sort_list_by_name(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Sort by name dec/inc [{:?}/{:?}]",
            key.sort_name_dec,
            key.sort_name_inc,
        ),
        CMD_GROUP_GENERAL
    )
}

pub fn sort_list_by_pid(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Sort by PID dec/inc [{:?}/{:?}]",
            key.sort_pid_dec,
            key.sort_pid_inc,
        ),
        CMD_GROUP_GENERAL
    )
}

pub fn sort_list_by_usage(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Sort by usage dec/inc [{:?}/{:?}]",
            key.sort_usage_dec,
            key.sort_usage_inc,
        ),
        CMD_GROUP_GENERAL
    )
}

pub fn follow_selection(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Toggle follow selection [{:?}]",
            key.follow_selection,
        ),
        CMD_GROUP_GENERAL
    )
}