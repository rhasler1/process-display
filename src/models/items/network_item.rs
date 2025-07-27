#[derive(Default)]
pub struct NetworkItem {
    tx: u64,
    rx: u64,
    total_tx: u64,
    total_rx: u64,
}

impl NetworkItem {
    pub fn new(
        tx: u64,
        rx: u64,
        total_tx: u64,
        total_rx: u64,
    ) -> Self {
        Self {
            tx,
            rx,
            total_tx,
            total_rx,
        }
    }

    // GETTERS 
    pub fn tx(&self) -> u64 {
        self.tx
    }

    pub fn rx(&self) -> u64 {
        self.rx
    }

    pub fn total_tx(&self) -> u64 {
        self.total_tx
    }

    pub fn total_rx(&self) -> u64 {
        self.total_rx
    }
}