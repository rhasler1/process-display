#[derive(Default, Clone, Debug)]
pub struct ProcessListItem {
    pid: u32,
    name: String,
    cpu_usage: f32,
    memory_usage: u64,
    start_time: u64,
    run_time: u64,
    accumulated_cpu_time: u64,
    status: String,
    path: String,
}

impl ProcessListItem {
    pub fn new(
        pid: u32,
        name: String,
        cpu_usage: f32,
        memory_usage: u64,
        start_time: u64,
        run_time: u64,
        accumulated_cpu_time: u64,
        status: String,
        path: String,
    ) -> Self {
        Self {
            pid,
            name,
            cpu_usage,
            memory_usage,
            start_time,
            run_time,
            accumulated_cpu_time,
            status,
            path,
        }
    }

    // match by name or pid
    //pub fn is_match(&self, filter_text: &str) -> bool {
    //    self.name.contains(filter_text) ||
    //    self.pid.to_string().contains(filter_text)
    //}

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cpu_usage(&self) -> f32 {
        self.cpu_usage
    }

    pub fn memory_usage(&self) -> u64 {
        self.memory_usage
    }

    pub fn start_time(&self) -> u64 {
        self.start_time
    }

    pub fn run_time(&self) -> u64 {
        self.run_time
    }

    pub fn run_time_hh_mm_ss(&self) -> String {
        let time_in_s = self.run_time;

        let ss =  time_in_s % 60;
        let mm = (time_in_s / 60) % 60;
        let hh = (time_in_s / 60) / 60;

        format!("{:0>2}:{:0>2}:{:0>2}", hh, mm, ss)
    }

    pub fn accumulated_cpu_time(&self) -> u64 {
        self.accumulated_cpu_time
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

// PartialEq is needed for comparison, e.g., calling contains
impl PartialEq for ProcessListItem {
    fn eq(&self, other: &Self) -> bool {
        self.pid.eq(&other.pid)
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
        assert_eq!(instance.start_time, 0);
        assert_eq!(instance.run_time, 0);
        assert_eq!(instance.accumulated_cpu_time, 0);
        assert!(String::is_empty(&instance.status));

        let instance = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));
        assert_eq!(instance.pid, 1);
        assert_eq!(instance.name, String::from("a"));
        assert_eq!(instance.cpu_usage, 1.0);
        assert_eq!(instance.memory_usage, 1);
        assert_eq!(instance.start_time, 0);
        assert_eq!(instance.run_time, 10);
        assert_eq!(instance.accumulated_cpu_time, 10);
        assert_eq!(instance.status, String::from("test"));
        
    }

    #[test]
    fn test_instance_functions() {
        let instance_0 = ProcessListItem::default();
        let instance_1 = ProcessListItem::new(1, String::from("a"), 1.0, 1, 0, 10, 10, String::from("test"), String::from("test"));

        assert_eq!(instance_0.pid(), instance_0.pid);
        assert_eq!(instance_0.name(), instance_0.name);
        assert_eq!(instance_0.cpu_usage(), instance_0.cpu_usage);
        assert_eq!(instance_0.memory_usage(), instance_0.memory_usage);
        assert_eq!(instance_0.start_time(), instance_0.start_time);
        assert_eq!(instance_0.run_time(), instance_0.run_time);
        assert_eq!(instance_0.accumulated_cpu_time(), instance_0.accumulated_cpu_time);
        assert_eq!(instance_0.status(), instance_0.status);
        //assert_eq!(instance_0.is_match(""), true);
        //assert_eq!(instance_0.is_match("a"), false);
        //assert_eq!(instance_0.is_match(&instance_0.pid.to_string()), true);

        assert_eq!(instance_1.pid(), instance_1.pid);
        assert_eq!(instance_1.name(), instance_1.name);
        assert_eq!(instance_1.cpu_usage(), instance_1.cpu_usage);
        assert_eq!(instance_1.memory_usage(), instance_1.memory_usage);
        assert_eq!(instance_0.start_time(), instance_0.start_time);
        assert_eq!(instance_0.run_time(), instance_0.run_time);
        assert_eq!(instance_0.accumulated_cpu_time(), instance_0.accumulated_cpu_time);
        assert_eq!(instance_0.status(), instance_0.status);
        //assert_eq!(instance_1.is_match("a"), true);
        //assert_eq!(instance_1.is_match("aa"), false);
        //assert_eq!(instance_1.is_match(&instance_1.pid.to_string()), true);
    }
}
