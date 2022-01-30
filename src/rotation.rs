use crate::vector::Vector3;
use crate::IntoMint;
use core::marker::PhantomData;

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

impl<T> IntoMint for Quaternion<T> {
    type MintType = Quaternion<T>;
}

impl<T> From<[T; 4]> for Quaternion<T> {
    fn from([x, y, z, s]: [T; 4]) -> Self {
        Quaternion {
            s,
            v: Vector3::from([x, y, z]),
        }
    }
}

impl<T> From<Quaternion<T>> for [T; 4] {
    fn from(quaternion: Quaternion<T>) -> [T; 4] {
        [quaternion.v.x, quaternion.v.y, quaternion.v.z, quaternion.s]
    }
}

impl<T> AsRef<[T; 4]> for Quaternion<T> {
    fn as_ref(&self) -> &[T; 4] {
        unsafe { ::core::mem::transmute(self) }
    }
}

impl<T> AsMut<[T; 4]> for Quaternion<T> {
    fn as_mut(&mut self) -> &mut [T; 4] {
        unsafe { ::core::mem::transmute(self) }
    }
}

#[cfg(feature = "serde")]
impl<T> ::serde::Serialize for Quaternion<T>
where
    T: ::serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        AsRef::<[T; 4]>::as_ref(self).serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T> ::serde::Deserialize<'de> for Quaternion<T>
where
    T: ::serde::Deserialize<'de>,
{
    fn deserialize<S>(deserializer: S) -> Result<Self, S::Error>
    where
        S: ::serde::Deserializer<'de>,
    {
        <[T; 4]>::deserialize(deserializer).map(Quaternion::<T>::from)
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

impl<T, B> From<EulerAngles<T, B>> for [T; 3] {
    fn from(euler: EulerAngles<T, B>) -> [T; 3] {
        [euler.a, euler.b, euler.c]
    }
}

#[cfg(feature = "serde")]
impl<T, B> ::serde::Serialize for EulerAngles<T, B>
where
    T: ::serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        [&self.a, &self.b, &self.c].serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T, B> ::serde::Deserialize<'de> for EulerAngles<T, B>
where
    T: ::serde::Deserialize<'de>,
{
    fn deserialize<S>(deserializer: S) -> Result<Self, S::Error>
    where
        S: ::serde::Deserializer<'de>,
    {
        <[T; 3]>::deserialize(deserializer).map(EulerAngles::<T, B>::from)
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
