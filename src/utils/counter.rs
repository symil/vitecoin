pub struct Counter {
    next_value: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { next_value: 1 }
    }

    pub fn next(&mut self) -> u32 {
        let result = self.next_value;

        self.next_value += 1;

        result
    }
}
