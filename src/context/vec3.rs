use memflow::prelude::v1::Pod;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

unsafe impl Pod for Vec3 {}
