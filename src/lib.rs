// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-kernel-panic: kernel panic component model.
//!
//! Real implementation: kernels, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kernel {
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
pub enum KernelPanicError {
    EmptyKernelName,
    DuplicateKernel(String),
    EmptyEventSubject,
}

impl fmt::Display for KernelPanicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKernelName => write!(f, "kernel name must not be empty"),
            Self::DuplicateKernel(id) => write!(f, "duplicate kernel {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for KernelPanicError {}

pub struct KernelPanic {
    pub name: String,
    kernels: BTreeMap<String, Kernel>,
    events: BTreeMap<String, Event>,
}

impl KernelPanic {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kernels: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_kernel(&mut self, item: Kernel) -> Result<(), KernelPanicError> {
        if item.name.trim().is_empty() {
            return Err(KernelPanicError::EmptyKernelName);
        }
        let id = item.id.clone();
        if self.kernels.contains_key(&id) {
            return Err(KernelPanicError::DuplicateKernel(id.clone()));
        }
        self.kernels.insert(id, item);
        Ok(())
    }

    pub fn kernel_count(&self) -> usize {
        self.kernels.len()
    }

    pub fn get_kernel(&self, id: &str) -> Option<&Kernel> {
        self.kernels.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), KernelPanicError> {
        if event.subject.trim().is_empty() {
            return Err(KernelPanicError::EmptyEventSubject);
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

    fn sample_kernel(id: &str, name: &str) -> Kernel {
        Kernel {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = KernelPanic::new("KernelPanic");
        assert_eq!(m.name, "KernelPanic");
        assert_eq!(m.kernel_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_kernel() {
        let mut m = KernelPanic::new("KernelPanic");
        m.add_kernel(sample_kernel("a1", "Alpha")).unwrap();
        assert_eq!(m.kernel_count(), 1);
        assert!(m.get_kernel("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = KernelPanic::new("KernelPanic");
        assert!(m.add_kernel(sample_kernel("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = KernelPanic::new("KernelPanic");
        m.add_kernel(sample_kernel("x", "One")).unwrap();
        assert!(m.add_kernel(sample_kernel("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = KernelPanic::new("KernelPanic");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
