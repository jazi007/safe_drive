// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
pub const STATUS_NO_FIX: i8 = -1; // unable to fix position
pub const STATUS_FIX: i8 = 0; // unaugmented fix
pub const STATUS_SBAS_FIX: i8 = 1; // with satellite-based augmentation
pub const STATUS_GBAS_FIX: i8 = 2; // with ground-based augmentation
pub const SERVICE_GPS: u16 = 1;
pub const SERVICE_GLONASS: u16 = 2;
pub const SERVICE_COMPASS: u16 = 4; // includes BeiDou.
pub const SERVICE_GALILEO: u16 = 8;

extern "C" {
    fn sensor_msgs__msg__NavSatStatus__init(msg: *mut NavSatStatus) -> bool;
    fn sensor_msgs__msg__NavSatStatus__fini(msg: *mut NavSatStatus);
    fn sensor_msgs__msg__NavSatStatus__Sequence__init(msg: *mut NavSatStatusSequence, size: usize) -> bool;
    fn sensor_msgs__msg__NavSatStatus__Sequence__fini(msg: *mut NavSatStatusSequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct NavSatStatus {
    pub status: i8,
    pub service: u16,
}

impl NavSatStatus {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__NavSatStatus__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for NavSatStatus {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__NavSatStatus__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct NavSatStatusSequence {
    data: *mut NavSatStatus,
    size: usize,
    capacity: usize,
}

impl NavSatStatusSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__NavSatStatus__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[NavSatStatus]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [NavSatStatus]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for NavSatStatusSequence {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__NavSatStatus__Sequence__fini(self) };
    }
}

