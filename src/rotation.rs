use vector::Vector3;


/// Euler rotation angles in 3D space.
#[repr(C)]
pub struct EulerAngles<T> {
    /// Angle of rotation around X axis in range [-pi, pi] (_pitch_).
    pub x: T,
    /// Angle of rotation around Y axis in range [-pi/2, pi/2] (_yaw_).
    pub y: T,
    /// Angle of rotation around Z axis in range [-pi, pi] (_roll_).
    pub z: T,
}

impl<T: Clone> From<[T; 3]> for EulerAngles<T> {
    fn from(v: [T; 3]) -> Self {
        EulerAngles {
            x: v[0].clone(),
            y: v[1].clone(),
            z: v[2].clone(),
        }
    }
}

impl<T> Into<[T; 3]> for EulerAngles<T> {
    fn into(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}


/// Standard quaternion represented by the scalar and vector parts.
/// Useful for representing rotation in 3D space.
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
