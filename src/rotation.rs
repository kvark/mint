use core::marker::PhantomData;
use vector::Vector3;


/// Standard quaternion represented by the scalar and vector parts.
/// Useful for representing rotation in 3D space.
/// Corresponds to a right-handed rotation matrix.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct Quaternion<T> {
    /// Vector part of a quaternion.
    pub v: Vector3<T>,
    /// Scalar part of a quaternion.
    pub s: T,
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Quaternion<T> {}
#[cfg(feature = "bytemuck")]
unsafe impl<T: Copy + 'static> bytemuck::Pod for Quaternion<T> {}

impl<T> From<[T; 4]> for Quaternion<T> {
    fn from([x, y, z, s]: [T; 4]) -> Self {
        Quaternion {
            s,
            v: Vector3::from([x, y, z]),
        }
    }
}

impl<T> Into<[T; 4]> for Quaternion<T> {
    fn into(self) -> [T; 4] {
        [self.v.x, self.v.y, self.v.z, self.s]
    }
}

impl<T> AsRef<[T; 4]> for Quaternion<T> {
    fn as_ref(&self) -> &[T; 4] { unsafe { ::core::mem::transmute(self) } }
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
    pub marker: PhantomData<B>,
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

#[cfg(feature = "bytemuck")]
unsafe impl<T, B> bytemuck::Zeroable for EulerAngles<T, B> {}
#[cfg(feature = "bytemuck")]
unsafe impl<T: Copy + 'static, B: Copy + 'static> bytemuck::Pod for EulerAngles<T, B> {}

impl<T, B> From<[T; 3]> for EulerAngles<T, B> {
    fn from([a, b, c]: [T; 3]) -> Self {
        EulerAngles {
            a,
            b,
            c,
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
