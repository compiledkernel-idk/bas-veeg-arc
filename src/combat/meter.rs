pub struct MeterManager {
    pub current: f32,
    pub maximum: f32,
    pub gain_multiplier: f32,
    pub segments: u32,
}

#[derive(Clone, Debug)]
pub enum MeterGainType {
    DamageDealt(f32),
    DamageReceived(f32),
    BlockSuccessful,
    ParrySuccessful,
    ComboExtended,
    SpecialMove,
}

impl MeterManager {
    pub fn new(max_meter: f32, segments: u32) -> Self {
        Self {
            current: 0.0,
            maximum: max_meter,
            gain_multiplier: 1.0,
            segments,
        }
    }

    pub fn gain(&mut self, gain_type: MeterGainType) {
        let amount = match gain_type {
            MeterGainType::DamageDealt(dmg) => dmg * 0.5,
            MeterGainType::DamageReceived(dmg) => dmg * 0.3,
            MeterGainType::BlockSuccessful => 5.0,
            MeterGainType::ParrySuccessful => 15.0,
            MeterGainType::ComboExtended => 3.0,
            MeterGainType::SpecialMove => 10.0,
        };

        self.current = (self.current + amount * self.gain_multiplier).min(self.maximum);
    }

    pub fn consume(&mut self, amount: f32) -> bool {
        if self.current >= amount {
            self.current -= amount;
            true
        } else {
            false
        }
    }

    pub fn consume_segments(&mut self, segments: u32) -> bool {
        let segment_cost = self.maximum / self.segments as f32;
        let total_cost = segment_cost * segments as f32;

        self.consume(total_cost)
    }

    pub fn get_filled_segments(&self) -> u32 {
        let segment_size = self.maximum / self.segments as f32;
        (self.current / segment_size) as u32
    }

    pub fn get_percentage(&self) -> f32 {
        self.current / self.maximum
    }

    pub fn is_full(&self) -> bool {
        self.current >= self.maximum
    }

    pub fn reset(&mut self) {
        self.current = 0.0;
    }
}
