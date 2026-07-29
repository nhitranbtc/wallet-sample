use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RetryPolicy {
    base: Duration,
    max: Duration,
    attempts: u32,
    jitter_seed: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::new(0)
    }
}

impl RetryPolicy {
    pub fn new(jitter_seed: u64) -> Self {
        Self {
            base: Duration::from_millis(250),
            max: Duration::from_secs(5),
            attempts: 3,
            jitter_seed,
        }
    }

    pub fn max_attempts(&self) -> u32 {
        self.attempts
    }

    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let exponential = self.base.saturating_mul(1_u32 << attempt.min(5));
        let jitter = Duration::from_millis(self.seeded_jitter(attempt));
        (exponential + jitter).min(self.max)
    }

    fn seeded_jitter(&self, attempt: u32) -> u64 {
        let mut value = self
            .jitter_seed
            .wrapping_add(u64::from(attempt).wrapping_mul(0x9e37_79b9_7f4a_7c15));
        value ^= value >> 30;
        value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value ^= value >> 27;
        value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
        (value ^ (value >> 31)) % 100
    }
}
