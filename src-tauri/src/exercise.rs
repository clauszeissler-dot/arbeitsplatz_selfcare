use lotse_core::Hint;
use std::time::{Duration, SystemTime};

pub struct Exercise {
    pub hint: Hint,
    pub deadline: Option<SystemTime>,
    pub finished: bool,
    pub sound: bool,
}

impl Exercise {
    pub fn remaining(&self, now: SystemTime) -> u32 {
        self.deadline
            .and_then(|d| d.duration_since(now).ok())
            .map(|d| d.as_secs().saturating_add(u64::from(d.subsec_nanos() > 0)) as u32)
            .unwrap_or(0)
    }

    pub fn start(&mut self, now: SystemTime, sound: bool) -> Result<(), String> {
        if self.deadline.is_some() {
            return Err("Der Timer wurde bereits gestartet.".into());
        }
        let seconds = self
            .hint
            .duration_seconds
            .filter(|s| *s > 0 && *s <= 43200)
            .ok_or("Dieser Hinweis hat keine Übungsdauer.")?;
        self.deadline = Some(now + Duration::from_secs(seconds as u64));
        self.sound = sound;
        Ok(())
    }

    pub fn finish_if_due(&mut self, now: SystemTime) -> bool {
        if self.deadline.is_some() && !self.finished && self.remaining(now) == 0 {
            self.finished = true;
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lotse_core::{HintKind, Level};
    #[test]
    fn countdown_rounds_up_survives_sleep_and_finishes_once() {
        let now = SystemTime::UNIX_EPOCH;
        let mut e = Exercise {
            hint: Hint {
                kind: HintKind::Eyes,
                level: Level::Green,
                title: "Augen".into(),
                body: String::new(),
                actions: vec![],
                at: 0,
                duration_seconds: Some(20),
            },
            deadline: None,
            finished: false,
            sound: false,
        };
        assert!(!e.finish_if_due(now));
        e.start(now, true).unwrap();
        assert!(e.start(now, true).is_err());
        assert_eq!(e.remaining(now + Duration::from_millis(1)), 20);
        assert_eq!(e.remaining(now + Duration::from_secs(19)), 1);
        assert!(!e.finish_if_due(now + Duration::from_secs(19)));
        assert!(e.finish_if_due(now + Duration::from_secs(30)));
        assert!(!e.finish_if_due(now + Duration::from_secs(31)));
        e.deadline = None;
        assert!(!e.finish_if_due(now + Duration::from_secs(40)));
        e.hint.duration_seconds = None;
        assert!(e.start(now, true).is_err());
    }
}
