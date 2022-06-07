// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
pub const OK: u8 = 0;
pub const WARN: u8 = 1;
pub const ERROR: u8 = 2;
pub const STALE: u8 = 3;

extern "C" {
    fn diagnostic_msgs__msg__DiagnosticStatus__init(msg: *mut DiagnosticStatus) -> bool;
    fn diagnostic_msgs__msg__DiagnosticStatus__fini(msg: *mut DiagnosticStatus);
    fn diagnostic_msgs__msg__DiagnosticStatus__Sequence__init(msg: *mut DiagnosticStatusSequence, size: usize) -> bool;
    fn diagnostic_msgs__msg__DiagnosticStatus__Sequence__fini(msg: *mut DiagnosticStatusSequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticStatus {
    pub level: u8,
    pub name: crate::msg::RosString<0>,
    pub message: crate::msg::RosString<0>,
    pub hardware_id: crate::msg::RosString<0>,
    pub values: super::KeyValueSequence,
}

impl DiagnosticStatus {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__msg__DiagnosticStatus__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for DiagnosticStatus {
    fn drop(&mut self) {
        unsafe { diagnostic_msgs__msg__DiagnosticStatus__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticStatusSequence {
    data: *mut DiagnosticStatus,
    size: usize,
    capacity: usize,
}

impl DiagnosticStatusSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { diagnostic_msgs__msg__DiagnosticStatus__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[DiagnosticStatus]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [DiagnosticStatus]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for DiagnosticStatusSequence {
    fn drop(&mut self) {
        unsafe { diagnostic_msgs__msg__DiagnosticStatus__Sequence__fini(self) };
    }
}

