// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;

extern "C" {
    fn sensor_msgs__msg__MultiEchoLaserScan__init(msg: *mut MultiEchoLaserScan) -> bool;
    fn sensor_msgs__msg__MultiEchoLaserScan__fini(msg: *mut MultiEchoLaserScan);
    fn sensor_msgs__msg__MultiEchoLaserScan__Sequence__init(msg: *mut MultiEchoLaserScanSequence, size: usize) -> bool;
    fn sensor_msgs__msg__MultiEchoLaserScan__Sequence__fini(msg: *mut MultiEchoLaserScanSequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct MultiEchoLaserScan {
    pub header: std_msgs::msg::Header,
    pub angle_min: f32,
    pub angle_max: f32,
    pub angle_increment: f32,
    pub time_increment: f32,
    pub scan_time: f32,
    pub range_min: f32,
    pub range_max: f32,
    pub ranges: super::LaserEchoSequence,
    pub intensities: super::LaserEchoSequence,
}

impl MultiEchoLaserScan {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__MultiEchoLaserScan__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MultiEchoLaserScan {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__MultiEchoLaserScan__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MultiEchoLaserScanSequence {
    data: *mut MultiEchoLaserScan,
    size: usize,
    capacity: usize,
}

impl MultiEchoLaserScanSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__MultiEchoLaserScan__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[MultiEchoLaserScan]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [MultiEchoLaserScan]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for MultiEchoLaserScanSequence {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__MultiEchoLaserScan__Sequence__fini(self) };
    }
}

