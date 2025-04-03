#[derive(Clone)]
pub struct ProcessItemInfo {
    start_time: u64,
    run_time: u64,
    accumulated_cpu_time: u64,
    status: String,
}

impl ProcessItemInfo {
    pub fn new(
        start_time: u64, 
        run_time: u64, 
        accumulated_cpu_time: u64, 
        status: String,
    ) -> Self {
        Self {
            start_time, run_time, accumulated_cpu_time, status
        }
    }

    pub fn update(
        &mut self,
        start_time: u64, 
        run_time: u64, 
        accumulated_cpu_time: u64, 
        status: String,
    ) {
        self.start_time = start_time;
        self.run_time = run_time;
        self.accumulated_cpu_time = accumulated_cpu_time;
        self.status = status;
    }

    pub fn get_info(&self) -> Vec<String> {
        let info: Vec<String> = [
            self.start_time.to_string(),
            self.run_time.to_string(),
            self.accumulated_cpu_time.to_string(),
            self.status.clone(),
        ].to_vec();
        
        info
    }
}