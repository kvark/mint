use std::marker::PhantomData;
use vector::Vector3;


/// Standard quaternion represented by the scalar and vector parts.
/// Useful for representing rotation in 3D space.
/// Corresponds to a right-handed rotation matrix.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct Quaternion<T> {
    /// Scalar part of a quaternion.
    pub s: T,
    /// Vector part of a quaternion.
    pub v: Vector3<T>,
}

impl<T: Clone> From<[T; 4]> for Quaternion<T> {
    fn from(v: [T; 4]) -> Self {
        Quaternion {
            s: v[3].clone(),
            v: Vector3 {
                x: v[0].clone(),
                y: v[1].clone(),
                z: v[2].clone(),
            },
        }
    }
}

impl<T> Into<[T; 4]> for Quaternion<T> {
    fn into(self) -> [T; 4] {
        [self.v.x, self.v.y, self.v.z, self.s]
    }
}


/// Standard quaternion corresponding to a left-handed
/// rotation matrix. The exact association of the left-handed basis
/// that is encoded by this quaternion and a right-handed one
/// is presented by `B` (for "basis") generic parameter.
///
/// Read also:
/// https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation#Orientation
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

impl<T, B> Into<[T; 4]> for LeftQuaternion<T, B> {
    fn into(self) -> [T; 4] {
        [self.v.x, self.v.y, self.v.z, self.s]
    }
}


/// Abstract set of Euler angles in 3D space. The basis of angles
/// is defined by the generic parameter `B`.
///
/// Note: there are multiple notations of Euler angles. They are
/// split in two groups:
///   - intrinsic (also known as "Tait-Bryan angles"): rotate around local axis
///   - extrinsic (also known as "Proper Euler angles"): rotate around world axis
/// For each interpretation, different axis may be chosen in different order.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct EulerAngles<T, B> {
    /// First angle of rotation in range [-pi, pi] (_pitch_).
    pub a: T,
    /// Second angle of rotation around in range [-pi/2, pi/2] (_yaw_).
    pub b: T,
    /// Third angle of rotation in range [-pi, pi] (_roll_).
    pub c: T,
    /// Marker for the phantom basis.
    marker: PhantomData<B>,
}

/// Intrinsic rotation around X, then Y, then Z axis.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum IntraXYZ {}
/// Intrinsic rotation around Z, then X, then Z axis.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum IntraZXZ {}
/// Intrinsic rotation around Z, then Y, then X axis.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum IntraZYX {}
/// Extrinsic rotation around X, then Y, then Z axis.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum ExtraXYZ {}
/// Extrinsic rotation around Z, then X, then Z axis.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum ExtraZXZ {}
/// Extrinsic rotation around Z, then Y, then X axis.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum ExtraZYX {}

impl<T: Clone, B> From<[T; 3]> for EulerAngles<T, B> {
    fn from(v: [T; 3]) -> Self {
        EulerAngles {
            a: v[0].clone(),
            b: v[1].clone(),
            c: v[2].clone(),
            marker: PhantomData,
        }
    }
}

impl<T, B> Into<[T; 3]> for EulerAngles<T, B> {
    fn into(self) -> [T; 3] {
        [self.a, self.b, self.c]
    }
}

macro_rules! reverse {
    ($from:ident -> $to:ident) => {
        impl<T> From<EulerAngles<T, $from>> for EulerAngles<T, $to> {
            fn from(other: EulerAngles<T, $from>) -> Self {
                EulerAngles {
                    a: other.c,
                    b: other.b,
                    c: other.a,
                    marker: PhantomData,
                }
            }
        }
    };
    ($from:ident <-> $to:ident) => {
        reverse!($from -> $to);
        reverse!($to -> $from);
    };
}

reverse!(IntraXYZ <-> ExtraZYX);
reverse!(IntraZXZ <-> ExtraZXZ);
reverse!(IntraZYX <-> ExtraXYZ);
