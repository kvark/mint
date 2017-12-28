use std::marker::PhantomData;
use vector::Vector3;

pub use future_self::{
    IntraXYZ, IntraZXZ, IntraZYX,
    ExtraXYZ, ExtraZXZ, ExtraZYX,
    EulerAngles, Quaternion,
};

/// Standard quaternion corresponding to a left-handed
/// rotation matrix. The exact association of the left-handed basis
/// that is encoded by this quaternion and a right-handed one
/// is presented by `B` (for "basis") generic parameter.
///
/// Read also:
/// https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation#Orientation
#[deprecated]
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct LeftQuaternion<T, B> {
    /// Scalar part of a quaternion.
    pub s: T,
    /// Vector part of a quaternion.
    pub v: Vector3<T>,
    /// Marker for the phantom association with the right-handed basis.
    marker: PhantomData<B>,
}

/// Basis handedness change by mirroring X axis: x',y',z' = -x,y,z
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum MirrorX {}
/// Basis handedness change by mirroring Y axis: x',y',z' = x,-y,z
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum MirrorY {}
/// Basis handedness change by mirroring Z axis: x',y',z' = x,y,-z
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum MirrorZ {}
/// Basis handedness change by swapping X and Y axis: x',y',z' = y,x,z
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum SwapXY {}
/// Basis handedness change by swapping Y and Z axis: x',y',z' = x,z,y
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum SwapYZ {}
/// Basis handedness change by swapping Z and X axis: x',y',z' = z,y,x
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum SwapZX {}

#[allow(deprecated)]
impl<T: Clone, B> From<[T; 4]> for LeftQuaternion<T, B> {
    fn from(v: [T; 4]) -> Self {
        LeftQuaternion {
            s: v[3].clone(),
            v: Vector3 {
                x: v[0].clone(),
                y: v[1].clone(),
                z: v[2].clone(),
            },
            marker: PhantomData,
        }
    }
}

#[allow(deprecated)]
impl<T, B> Into<[T; 4]> for LeftQuaternion<T, B> {
    fn into(self) -> [T; 4] {
        [self.v.x, self.v.y, self.v.z, self.s]
    }
}
