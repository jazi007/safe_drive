// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;

extern "C" {
    fn std_msgs__msg__MultiArrayDimension__init(msg: *mut MultiArrayDimension) -> bool;
    fn std_msgs__msg__MultiArrayDimension__fini(msg: *mut MultiArrayDimension);
    fn std_msgs__msg__MultiArrayDimension__Sequence__init(msg: *mut MultiArrayDimensionSequence, size: usize) -> bool;
    fn std_msgs__msg__MultiArrayDimension__Sequence__fini(msg: *mut MultiArrayDimensionSequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct MultiArrayDimension {
    pub label: crate::msg::RosString<0>,
    pub size: u32,
    pub stride: u32,
}

impl MultiArrayDimension {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__MultiArrayDimension__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MultiArrayDimension {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__MultiArrayDimension__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MultiArrayDimensionSequence {
    data: *mut MultiArrayDimension,
    size: usize,
    capacity: usize,
}

impl MultiArrayDimensionSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__MultiArrayDimension__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[MultiArrayDimension]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [MultiArrayDimension]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for MultiArrayDimensionSequence {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__MultiArrayDimension__Sequence__fini(self) };
    }
}

