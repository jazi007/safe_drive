// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;

extern "C" {
    fn unique_identifier_msgs__msg__UUID__init(msg: *mut UUID) -> bool;
    fn unique_identifier_msgs__msg__UUID__fini(msg: *mut UUID);
    fn unique_identifier_msgs__msg__UUID__Sequence__init(msg: *mut UUIDSequence, size: usize) -> bool;
    fn unique_identifier_msgs__msg__UUID__Sequence__fini(msg: *mut UUIDSequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct UUID {
    pub uuid: [u8; 16],
}

impl UUID {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { unique_identifier_msgs__msg__UUID__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for UUID {
    fn drop(&mut self) {
        unsafe { unique_identifier_msgs__msg__UUID__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct UUIDSequence {
    data: *mut UUID,
    size: usize,
    capacity: usize,
}

impl UUIDSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { unique_identifier_msgs__msg__UUID__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[UUID]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [UUID]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for UUIDSequence {
    fn drop(&mut self) {
        unsafe { unique_identifier_msgs__msg__UUID__Sequence__fini(self) };
    }
}
