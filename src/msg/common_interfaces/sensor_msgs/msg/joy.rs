// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;

extern "C" {
    fn sensor_msgs__msg__Joy__init(msg: *mut Joy) -> bool;
    fn sensor_msgs__msg__Joy__fini(msg: *mut Joy);
    fn sensor_msgs__msg__Joy__Sequence__init(msg: *mut JoySequence, size: usize) -> bool;
    fn sensor_msgs__msg__Joy__Sequence__fini(msg: *mut JoySequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct Joy {
    pub header: std_msgs::msg::Header,
    pub axes: crate::msg::F32Seq<0>,
    pub buttons: crate::msg::I32Seq<0>,
}

impl Joy {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__Joy__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Joy {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__Joy__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct JoySequence {
    data: *mut Joy,
    size: usize,
    capacity: usize,
}

impl JoySequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__Joy__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Joy]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Joy]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for JoySequence {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__Joy__Sequence__fini(self) };
    }
}

