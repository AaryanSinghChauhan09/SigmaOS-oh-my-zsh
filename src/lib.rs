// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-oh-my-zsh: oh my zsh component model.
//!
//! Real implementation: zshs, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zsh {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OhMyZshError {
    EmptyZshName,
    DuplicateZsh(String),
    EmptyEventSubject,
}

impl fmt::Display for OhMyZshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyZshName => write!(f, "zsh name must not be empty"),
            Self::DuplicateZsh(id) => write!(f, "duplicate zsh {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for OhMyZshError {}

pub struct OhMyZsh {
    pub name: String,
    zshs: BTreeMap<String, Zsh>,
    events: BTreeMap<String, Event>,
}

impl OhMyZsh {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            zshs: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_zsh(&mut self, item: Zsh) -> Result<(), OhMyZshError> {
        if item.name.trim().is_empty() {
            return Err(OhMyZshError::EmptyZshName);
        }
        let id = item.id.clone();
        if self.zshs.contains_key(&id) {
            return Err(OhMyZshError::DuplicateZsh(id.clone()));
        }
        self.zshs.insert(id, item);
        Ok(())
    }

    pub fn zsh_count(&self) -> usize {
        self.zshs.len()
    }

    pub fn get_zsh(&self, id: &str) -> Option<&Zsh> {
        self.zshs.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), OhMyZshError> {
        if event.subject.trim().is_empty() {
            return Err(OhMyZshError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_zsh(id: &str, name: &str) -> Zsh {
        Zsh {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = OhMyZsh::new("OhMyZsh");
        assert_eq!(m.name, "OhMyZsh");
        assert_eq!(m.zsh_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_zsh() {
        let mut m = OhMyZsh::new("OhMyZsh");
        m.add_zsh(sample_zsh("a1", "Alpha")).unwrap();
        assert_eq!(m.zsh_count(), 1);
        assert!(m.get_zsh("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = OhMyZsh::new("OhMyZsh");
        assert!(m.add_zsh(sample_zsh("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = OhMyZsh::new("OhMyZsh");
        m.add_zsh(sample_zsh("x", "One")).unwrap();
        assert!(m.add_zsh(sample_zsh("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = OhMyZsh::new("OhMyZsh");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
