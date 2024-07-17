// This structure contains pertinent information to a Process.
#[derive(Default, Clone, Debug)]
pub struct ProcessListItem {
    pid: u32,
    name: String,
    cpu_usage: f32,
    memory_usage: u64,
}

impl ProcessListItem {
    // This function constructs a new instance of ProcessListItem with the parameters.
    pub fn new(pid: u32, name: String, cpu_usage: f32, memory_usage: u64) -> Self {
        Self {
            pid,
            name,
            cpu_usage,
            memory_usage,
        }
    }

    // This is a boolean function to determine if the name contained by the instance of ProcessListItem, &self,
    // matches the parameter filter_text.
    pub fn is_match(&self, filter_text: &str) -> bool {
        self.name.contains(filter_text)
    }

    // This function gets the pid of a ProcessListItem instance.
    pub fn pid(&self) -> u32 {
        self.pid.clone()
    }

    // This function gets the name of a ProcessListItem instance.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    // This function gets the cpu usage of a ProcessListItem instance.
    pub fn cpu_usage(&self) -> f32 {
        self.cpu_usage.clone()
    }

    // This function gets the memory usage of a ProcessListItem instance.
    pub fn memory_usage(&self) -> u64 {
        self.memory_usage.clone()
    }
}

// Here, I am implementing the trait Partial Equality for an instance of ProcessListItem.
// This is done so that ProcessListItems in a Vector can be iterated over and compared.
impl PartialEq for ProcessListItem {
    // This is a boolean function to determine if the ProcessListItem instance &self
    // is equal to the parameter other. The comparison is done by pid.
    fn eq(&self, other: &Self) -> bool {
        return self.pid().eq(&other.pid())
    }
}

#[cfg(test)]
pub mod test {
    use super::ProcessListItem;

    #[test]
    fn test_constructors() {
        let instance = ProcessListItem::default();
        assert_eq!(instance.pid, 0);
        assert!(String::is_empty(&instance.name));
        assert_eq!(instance.cpu_usage, 0.0);
        assert_eq!(instance.memory_usage, 0);

        let instance = ProcessListItem::new(1, String::from("a"), 1.0, 1);
        assert_eq!(instance.pid, 1);
        assert_eq!(instance.name, String::from("a"));
        assert_eq!(instance.cpu_usage, 1.0);
        assert_eq!(instance.memory_usage, 1);
    }

    #[test]
    fn test_instance_functions() {
        let instance_0 = ProcessListItem::default();
        let instance_1 = ProcessListItem::new(1, String::from("a"), 1.0, 1);

        assert_eq!(instance_0.pid(), instance_0.pid);
        assert_eq!(instance_0.name(), instance_0.name);
        assert_eq!(instance_0.cpu_usage(), instance_0.cpu_usage);
        assert_eq!(instance_0.memory_usage(), instance_0.memory_usage);
        assert_eq!(instance_0.is_match(""), true);
        assert_ne!(instance_0.is_match("a"), true);

        assert_eq!(instance_1.pid(), instance_1.pid);
        assert_eq!(instance_1.name(), instance_1.name);
        assert_eq!(instance_1.cpu_usage(), instance_1.cpu_usage);
        assert_eq!(instance_1.memory_usage(), instance_1.memory_usage);
        assert_eq!(instance_1.is_match("a"), true);
        assert_ne!(instance_1.is_match("aa"), true);
    }
}