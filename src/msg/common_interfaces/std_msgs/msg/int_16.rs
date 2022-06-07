// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;

extern "C" {
    fn std_msgs__msg__Int16__init(msg: *mut Int16) -> bool;
    fn std_msgs__msg__Int16__fini(msg: *mut Int16);
    fn std_msgs__msg__Int16__Sequence__init(msg: *mut Int16Sequence, size: usize) -> bool;
    fn std_msgs__msg__Int16__Sequence__fini(msg: *mut Int16Sequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct Int16 {
    pub data: i16,
}

impl Int16 {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int16__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Int16 {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Int16__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Int16Sequence {
    data: *mut Int16,
    size: usize,
    capacity: usize,
}

impl Int16Sequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int16__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Int16]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Int16]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for Int16Sequence {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Int16__Sequence__fini(self) };
    }
}

