// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;

extern "C" {
    fn std_msgs__msg__ColorRGBA__init(msg: *mut ColorRGBA) -> bool;
    fn std_msgs__msg__ColorRGBA__fini(msg: *mut ColorRGBA);
    fn std_msgs__msg__ColorRGBA__Sequence__init(msg: *mut ColorRGBASequence, size: usize) -> bool;
    fn std_msgs__msg__ColorRGBA__Sequence__fini(msg: *mut ColorRGBASequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct ColorRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ColorRGBA {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__ColorRGBA__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for ColorRGBA {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__ColorRGBA__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ColorRGBASequence {
    data: *mut ColorRGBA,
    size: usize,
    capacity: usize,
}

impl ColorRGBASequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__ColorRGBA__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[ColorRGBA]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [ColorRGBA]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for ColorRGBASequence {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__ColorRGBA__Sequence__fini(self) };
    }
}

