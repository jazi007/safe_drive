// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;

extern "C" {
    fn geometry_msgs__msg__TwistStamped__init(msg: *mut TwistStamped) -> bool;
    fn geometry_msgs__msg__TwistStamped__fini(msg: *mut TwistStamped);
    fn geometry_msgs__msg__TwistStamped__Sequence__init(msg: *mut TwistStampedSequence, size: usize) -> bool;
    fn geometry_msgs__msg__TwistStamped__Sequence__fini(msg: *mut TwistStampedSequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct TwistStamped {
    pub header: std_msgs::msg::Header,
    pub twist: Twist,
}

impl TwistStamped {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__TwistStamped__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for TwistStamped {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__TwistStamped__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TwistStampedSequence {
    data: *mut TwistStamped,
    size: usize,
    capacity: usize,
}

impl TwistStampedSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__TwistStamped__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[TwistStamped]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [TwistStamped]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for TwistStampedSequence {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__TwistStamped__Sequence__fini(self) };
    }
}
