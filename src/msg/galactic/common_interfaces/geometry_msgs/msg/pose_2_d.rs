// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;

extern "C" {
    fn geometry_msgs__msg__Pose2D__init(msg: *mut Pose2D) -> bool;
    fn geometry_msgs__msg__Pose2D__fini(msg: *mut Pose2D);
    fn geometry_msgs__msg__Pose2D__Sequence__init(msg: *mut Pose2DSequence, size: usize) -> bool;
    fn geometry_msgs__msg__Pose2D__Sequence__fini(msg: *mut Pose2DSequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct Pose2D {
    pub x: f64,
    pub y: f64,
    pub theta: f64,
}

impl Pose2D {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Pose2D__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Pose2D {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Pose2D__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Pose2DSequence {
    data: *mut Pose2D,
    size: usize,
    capacity: usize,
}

impl Pose2DSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Pose2D__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Pose2D]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Pose2D]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for Pose2DSequence {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Pose2D__Sequence__fini(self) };
    }
}
