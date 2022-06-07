// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;

extern "C" {
    fn actionlib_msgs__msg__GoalID__init(msg: *mut GoalID) -> bool;
    fn actionlib_msgs__msg__GoalID__fini(msg: *mut GoalID);
    fn actionlib_msgs__msg__GoalID__Sequence__init(msg: *mut GoalIDSequence, size: usize) -> bool;
    fn actionlib_msgs__msg__GoalID__Sequence__fini(msg: *mut GoalIDSequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct GoalID {
    pub stamp: builtin_interfaces__msg__Time,
    pub id: crate::msg::RosString<0>,
}

impl GoalID {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { actionlib_msgs__msg__GoalID__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for GoalID {
    fn drop(&mut self) {
        unsafe { actionlib_msgs__msg__GoalID__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct GoalIDSequence {
    data: *mut GoalID,
    size: usize,
    capacity: usize,
}

impl GoalIDSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { actionlib_msgs__msg__GoalID__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[GoalID]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [GoalID]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for GoalIDSequence {
    fn drop(&mut self) {
        unsafe { actionlib_msgs__msg__GoalID__Sequence__fini(self) };
    }
}

