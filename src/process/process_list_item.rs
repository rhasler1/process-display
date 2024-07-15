// This structure contains pertinent information to a system's CPU.
#[derive(Default, Clone, Debug)]
pub struct CpuInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
}

impl CpuInfo {
    // This function constructs a `new` instance CpuInfo.
    pub fn new(pid: u32, name: String, cpu_usage: f32) -> Self {
        Self {
            pid,
            name,
            cpu_usage,
        }
    }
}

// This structure contains pertinent information to a system's Memory.
#[derive(Default, Clone, Debug)]
pub struct MemoryInfo {
    pub pid: u32,
    pub name: String,
    pub memory_usage: u64,
}

impl MemoryInfo {
    // This function constructs a `new` instance of MemoryInfo.
    pub fn new(pid: u32, name: String, memory_usage: u64) -> Self {
        Self {
            pid,
            name,
            memory_usage,
        }
    }
}

// This enumerator can hold either a `CpuInfo` or `MemoryInfo` instance.
#[derive(Clone, Debug)]
pub enum ProcessListItem {
    Cpu(CpuInfo),
    Memory(MemoryInfo),
}

impl ProcessListItem {
    // This is a boolean function to determine if the enumerator instance &self is Cpu(CpuInfo).
    pub const fn is_cpu(&self) -> bool {
        matches!(self, Self::Cpu { .. })
    }

    // This is a boolean function to determine if the enumerator instance &self is Memory(MemoryInfo).
    pub const fn is_memory(&self) -> bool {
        matches!(self, Self::Memory { .. })
    }

    // This is a boolean function to determine if the name contained by the instance of ProcessListItem, &self,
    // matches the parameter filter_text.
    pub fn is_match(&self, filter_text: &str) -> bool {
        match self {
            Self::Cpu(cpu) => cpu.name.contains(filter_text),
            Self::Memory(memory) => memory.name.contains(filter_text),
        }
    }

    // This function gets the pid of a ProcessListItem instance.
    pub fn pid(&self) -> u32 {
        match self {
            Self::Cpu(cpu) => cpu.pid.clone(),
            Self::Memory(memory) => memory.pid.clone(),
        }
    }

    // This function gets the name of a ProcessListItem instance.
    pub fn name(&self) -> String {
        match self {
            Self::Cpu(cpu) => cpu.name.clone(),
            Self::Memory(memory) => memory.name.clone(),
        }
    }

    // This function gets the cpu usage of a ProcessListItem instance
    // if and only if the ProcessListItem holds a `CpuInfo` instance.
    pub fn cpu_usage(&self) -> Option<f32> {
        match self {
            Self::Cpu(cpu) => Some(cpu.cpu_usage.clone()),
            Self::Memory(_memory) => None,
        }
    }

    // This function gets the memory usage of a ProcessListItem instance
    // if and only if the ProcessListItem holds a `MemoryInfo` instance.
    pub fn memory_usage(&self) -> Option<u64> {
        match self {
            Self::Cpu(_cpu) => None,
            Self::Memory(memory) => Some(memory.memory_usage.clone()),
        }
    }
}

// Here, I am implementing the trait Partial Equality for an instance of ProcessListItem.
// This is done so that ProcessListItems in a Vector can be iterated over and compared.
impl PartialEq for ProcessListItem {
    // This is a boolean function to determine if the ProcessListItem instance &self
    // is equal to the parameter other. The comparison is done by process identification.
    fn eq(&self, other: &Self) -> bool {
        return self.pid().eq(&other.pid())
    }
}

#[cfg(test)]
pub mod test {
    use super::CpuInfo;
    use super::MemoryInfo;
    use super::ProcessListItem;

    #[test]
    fn test_construct_cpuinfo() {
        let instance = CpuInfo::default();
        assert!(String::is_empty(&instance.name));
        assert_eq!(instance.pid, 0);
        assert_eq!(instance.cpu_usage, 0.0);

        let instance = CpuInfo::new(1, String::from("a"), 1.0);
        assert_eq!(instance.pid, 1);
        assert_eq!(instance.name, String::from("a"));
        assert_eq!(instance.cpu_usage, 1.0);
    }

    #[test]
    fn test_construct_memoryinfo() {
        let instance = MemoryInfo::default();
        assert!(String::is_empty(&instance.name));
        assert_eq!(instance.pid, 0);
        assert_eq!(instance.memory_usage, 0);

        let instance = MemoryInfo::new(1, String::from("a"), 1);
        assert_eq!(instance.pid, 1);
        assert_eq!(instance.name, String::from("a"));
        assert_eq!(instance.memory_usage, 1);
    }

    #[test]
    fn test_processitem_cpuinfo() {
        let instance_info = CpuInfo::new(1, String::from("a"), 1.0);
        let instance = ProcessListItem::Cpu(instance_info);
        assert_eq!(instance.is_cpu(), true);
        assert_eq!(instance.is_memory(), false);
        assert_eq!(instance.is_match("a"), true);
        assert_eq!(instance.is_match("b"), false);
        assert_eq!(instance.pid(), 1);
        assert_ne!(instance.pid(), 2);
        assert_eq!(instance.name(), String::from("a"));
        assert_ne!(instance.name(), String::from("b"));
        assert_eq!(instance.cpu_usage(), Some(1.0));
        assert_ne!(instance.cpu_usage(), Some(2.0));
        assert_eq!(instance.memory_usage(), None);
    }

    #[test]
    fn test_processitem_memoryinfo() {
        let instance_info = MemoryInfo::new(99, String::from("aa"), 123456789);
        let instance = ProcessListItem::Memory(instance_info);
        assert_eq!(instance.is_memory(), true);
        assert_eq!(instance.is_cpu(), false);
        assert_eq!(instance.is_match("aa"), true);
        assert_eq!(instance.is_match("bb"), false);
        assert_eq!(instance.pid(), 99);
        assert_ne!(instance.pid(), 98);
        assert_eq!(instance.name(), String::from("aa"));
        assert_ne!(instance.name(), String::from("bb"));
        assert_eq!(instance.memory_usage(), Some(123456789));
        assert_ne!(instance.memory_usage(), Some(0));
        assert_eq!(instance.cpu_usage(), None);
    }
}