use crate::config::{KeyConfig, MouseConfig};

static CMD_GROUP_GENERAL: &str = "-- General --";
static CMD_GROUP_PROCESS: &str = "-- Process --";
static CMD_GROUP_CPU: &str = "-- CPU --";
static CMD_GROUP_MEMORY: &str = "-- Memory --";
static CMD_GROUP_NETWORK: &str = "-- Network --";

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

pub fn exit_popup(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Exit current screen [{:?}]",
            key.exit,
        ),
        CMD_GROUP_GENERAL
    )
}

pub fn help(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Help [{:?}]",
            key.help,
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

// Process specific::begin
pub fn sort_list_by_name(key: &KeyConfig, mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!(
            "Sort by name toggle [{:?}]",
            key.sort_name_toggle,
        ),
        CMD_GROUP_PROCESS
    )
}

pub fn sort_list_by_pid(key: &KeyConfig, mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!(
            "Sort by PID toggle [{:?}]",
            key.sort_pid_toggle
        ),
        CMD_GROUP_PROCESS
    )
}

pub fn sort_list_by_cpu_usage(key: &KeyConfig, mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!(
            "Sort by cpu usage toggle [{:?}]",
            key.sort_cpu_toggle
        ),
        CMD_GROUP_PROCESS
    )
}

pub fn sort_list_by_memory_usage(key: &KeyConfig, mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!(
            "Sort by memory usage toggle [{:?}]",
            key.sort_memory_toggle
        ),
        CMD_GROUP_PROCESS
    )
}

pub fn select_process(key: &KeyConfig, mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!(
            "Select process by mouse [{:?}] | [{:?}/{:?}]",
            mouse.left_click, mouse.scroll_down, mouse.scroll_up,
        ),
        CMD_GROUP_PROCESS
    )
}
// Process specific::end