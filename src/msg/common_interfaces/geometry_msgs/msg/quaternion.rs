// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;

extern "C" {
    fn geometry_msgs__msg__Quaternion__init(msg: *mut Quaternion) -> bool;
    fn geometry_msgs__msg__Quaternion__fini(msg: *mut Quaternion);
    fn geometry_msgs__msg__Quaternion__Sequence__init(msg: *mut QuaternionSequence, size: usize) -> bool;
    fn geometry_msgs__msg__Quaternion__Sequence__fini(msg: *mut QuaternionSequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Quaternion {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Quaternion__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Quaternion {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Quaternion__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct QuaternionSequence {
    data: *mut Quaternion,
    size: usize,
    capacity: usize,
}

impl QuaternionSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Quaternion__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Quaternion]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Quaternion]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for QuaternionSequence {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Quaternion__Sequence__fini(self) };
    }
}

