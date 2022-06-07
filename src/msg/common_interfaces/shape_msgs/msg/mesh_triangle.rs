// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;

extern "C" {
    fn shape_msgs__msg__MeshTriangle__init(msg: *mut MeshTriangle) -> bool;
    fn shape_msgs__msg__MeshTriangle__fini(msg: *mut MeshTriangle);
    fn shape_msgs__msg__MeshTriangle__Sequence__init(msg: *mut MeshTriangleSequence, size: usize) -> bool;
    fn shape_msgs__msg__MeshTriangle__Sequence__fini(msg: *mut MeshTriangleSequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct MeshTriangle {
    pub vertex_indices: [u32; 3],
}

impl MeshTriangle {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { shape_msgs__msg__MeshTriangle__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MeshTriangle {
    fn drop(&mut self) {
        unsafe { shape_msgs__msg__MeshTriangle__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MeshTriangleSequence {
    data: *mut MeshTriangle,
    size: usize,
    capacity: usize,
}

impl MeshTriangleSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { shape_msgs__msg__MeshTriangle__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[MeshTriangle]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [MeshTriangle]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for MeshTriangleSequence {
    fn drop(&mut self) {
        unsafe { shape_msgs__msg__MeshTriangle__Sequence__fini(self) };
    }
}

