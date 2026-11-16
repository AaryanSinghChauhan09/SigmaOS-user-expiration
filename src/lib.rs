// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-user-expiration: user expiration component model.
//!
//! Real implementation: users, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
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
pub enum UserExpirationError {
    EmptyUserName,
    DuplicateUser(String),
    EmptyEventSubject,
}

impl fmt::Display for UserExpirationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyUserName => write!(f, "user name must not be empty"),
            Self::DuplicateUser(id) => write!(f, "duplicate user {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for UserExpirationError {}

pub struct UserExpiration {
    pub name: String,
    users: BTreeMap<String, User>,
    events: BTreeMap<String, Event>,
}

impl UserExpiration {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            users: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_user(&mut self, item: User) -> Result<(), UserExpirationError> {
        if item.name.trim().is_empty() {
            return Err(UserExpirationError::EmptyUserName);
        }
        let id = item.id.clone();
        if self.users.contains_key(&id) {
            return Err(UserExpirationError::DuplicateUser(id.clone()));
        }
        self.users.insert(id, item);
        Ok(())
    }

    pub fn user_count(&self) -> usize {
        self.users.len()
    }

    pub fn get_user(&self, id: &str) -> Option<&User> {
        self.users.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), UserExpirationError> {
        if event.subject.trim().is_empty() {
            return Err(UserExpirationError::EmptyEventSubject);
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

    fn sample_user(id: &str, name: &str) -> User {
        User {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = UserExpiration::new("UserExpiration");
        assert_eq!(m.name, "UserExpiration");
        assert_eq!(m.user_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_user() {
        let mut m = UserExpiration::new("UserExpiration");
        m.add_user(sample_user("a1", "Alpha")).unwrap();
        assert_eq!(m.user_count(), 1);
        assert!(m.get_user("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = UserExpiration::new("UserExpiration");
        assert!(m.add_user(sample_user("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = UserExpiration::new("UserExpiration");
        m.add_user(sample_user("x", "One")).unwrap();
        assert!(m.add_user(sample_user("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = UserExpiration::new("UserExpiration");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
