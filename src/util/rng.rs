pub struct SeededRng {
    state: u64,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        Self {
            state: seed ^ 0x5DEECE66D,
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        (self.state >> 16) as u32
    }

    pub fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }

    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as u32;
        min + (self.next_u32() % range) as i32
    }

    pub fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        min + self.next_f32() * (max - min)
    }

    pub fn chance(&mut self, probability: f32) -> bool {
        self.next_f32() < probability
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        let len = slice.len();
        for i in 0..len {
            let j = self.range(i as i32, len as i32) as usize;
            slice.swap(i, j);
        }
    }
}
