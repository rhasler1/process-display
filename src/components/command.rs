use crate::config::{KeyConfig, MouseConfig};

static CMD_GROUP_GENERAL: &str = "-- General --";
static CMD_GROUP_PROCESS: &str = "-- Process --";
static CMD_GROUP_CHART: &str = "-- Chart --";
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

pub fn select_component_key(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Rotate through components [{:?}]",
            key.tab
        ),
        CMD_GROUP_GENERAL
    )
}

pub fn select_component_mouse(mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!(
            "Select component to interact with [{:?}]",
            mouse.left_click
        ),
        CMD_GROUP_GENERAL
    )
}

pub fn move_selection_by_key(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Move selection up/down key [Arrow({:?})/Arrow({:?})]",
            key.move_up, key.move_down
        ),
        CMD_GROUP_GENERAL,
    )
}

pub fn move_selection_by_mouse(mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!(
            "Move selection up/down mouse [{:?}/{:?}]",
            mouse.scroll_up, mouse.scroll_down
        ),
        CMD_GROUP_GENERAL,
    )
}

pub fn move_selection_by_key_top_bottom(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Move selection to top/bottom [{:?}/{:?}]",
            key.move_top, key.move_bottom,
        ),
        CMD_GROUP_GENERAL,
    )
}

pub fn enter_filter_state_by_key(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Switch focus to Process Filter [{:?}]",
            key.filter,
        ),
        CMD_GROUP_PROCESS,
    )
}

pub fn exit_filter_state_by_key(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Switch focus to Process List [{:?}]",
            key.enter
        ),
        CMD_GROUP_PROCESS,
    )
}

pub fn exit_app_by_key(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Exit application [{:?}]",
            key.exit,
        ),
        CMD_GROUP_GENERAL,
    )
}

pub fn help(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Open and close help screen [{:?}]",
            key.help,
        ),
        CMD_GROUP_GENERAL,
    )
}

/*pub fn terminate_process(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Terminate selected process [{:?}]",
            key.terminate,
        ),
        CMD_GROUP_GENERAL
    )
}*/

// Process specific::begin
pub fn sort_list_by_name(key: &KeyConfig, mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!(
            "Sort by name toggle [{:?}]",
            key.sort_name_toggle,
        ),
        CMD_GROUP_PROCESS,
    )
}

pub fn sort_list_by_pid(key: &KeyConfig, mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!(
            "Sort by PID toggle [{:?}]",
            key.sort_pid_toggle
        ),
        CMD_GROUP_PROCESS,
    )
}

pub fn sort_list_by_cpu_usage(key: &KeyConfig, mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!(
            "Sort by CPU usage toggle [{:?}]",
            key.sort_cpu_toggle
        ),
        CMD_GROUP_PROCESS,
    )
}

pub fn sort_list_by_memory_usage(key: &KeyConfig, mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!(
            "Sort by memory usage toggle [{:?}]",
            key.sort_memory_toggle
        ),
        CMD_GROUP_PROCESS,
    )
}

pub fn sort_list_by_status(key: &KeyConfig, mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!(
            "Sort by status toggle [{:?}]",
            key.sort_status_toggle
        ),
        CMD_GROUP_PROCESS,
    )
}

pub fn sort_list_by_runtime(key: &KeyConfig, mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!(
            "Sort by runtime toggle [{:?}]",
            key.sort_runtime_toggle,
        ),
        CMD_GROUP_PROCESS,
    )
}

pub fn select_row(mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!(
            "Move selection with mouse [{:?}]",
            mouse.left_click,
        ),
        CMD_GROUP_PROCESS,
    )
}

pub fn select_process(key: &KeyConfig, mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!(
            "Select process by mouse [{:?}] | [{:?}/{:?}]",
            mouse.left_click, mouse.scroll_down, mouse.scroll_up,
        ),
        CMD_GROUP_PROCESS,
    )
}

pub fn zoom_chart_by_key(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Zoom chart in/out key [{:?}/{:?}]",
            key.move_up,
            key.move_down,
        ),
        CMD_GROUP_CHART
    )
}

pub fn zoom_chart_by_mouse(mouse: &MouseConfig) -> CommandText {
    CommandText::new(
        format!{
            "Zoom chart in/out mouse [{:?}/{:?}]",
            mouse.scroll_up,
            mouse.scroll_down,
        },
        CMD_GROUP_CHART
    )
}

pub fn pan_chart_by_key(key: &KeyConfig) -> CommandText {
    CommandText::new(
        format!(
            "Pan chart left/right [{:?}/{:?}]",
            key.move_left,
            key.move_right,
        ),
        CMD_GROUP_CHART
    )
}

// Process specific::end