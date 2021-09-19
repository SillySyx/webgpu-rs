use std::{
    error::Error,
    time::{Duration, SystemTime},
};

pub struct FrameTime {
    frame_time: SystemTime,
    duration_since_last_frame: Duration,
    target_frame_time: Option<u64>,
}

impl FrameTime {
    pub fn new(target_frame_time: Option<u64>) -> Self {
        Self {
            frame_time: SystemTime::now(),
            duration_since_last_frame: Duration::from_millis(0),
            target_frame_time,
        }
    }

    pub fn update(&mut self) -> Result<u128, Box<dyn Error>> {
        let now = SystemTime::now();

        self.duration_since_last_frame = now.duration_since(self.frame_time)?;

        self.frame_time = now;

        Ok(self.duration_since_last_frame.as_millis())
    }

    pub fn calc_sleep_duration(&self) -> Option<Duration> {
        if let Some(target_frame_time) = self.target_frame_time {
            let now = SystemTime::now();

            let duration_since_last_frame = now
                .duration_since(self.frame_time)
                .expect("What is time?");
    
            let target_duration = Duration::from_millis(target_frame_time);
            if duration_since_last_frame > target_duration {
                return None;
            }
    
            let duration_to_sleep = target_duration - duration_since_last_frame;
            if duration_to_sleep.as_millis() > 0 {
                return Some(duration_to_sleep);
            }
        }
        
        None
    }
}
