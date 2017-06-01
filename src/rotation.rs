use vector::Vector3;


/// Tait-Bryan rotation angles in ZYX convention, that is:
///   - first, rotation around Z axis is done
///   - next, rotation around the new Y axis is done
///   - finally, rotation around the new X axis is done
#[repr(C)]
pub struct TaitBryanAngles<T> {
    /// Angle of rotation around X axis in range [-pi, pi] (_pitch_).
    pub x: T,
    /// Angle of rotation around Y axis in range [-pi/2, pi/2] (_yaw_).
    pub y: T,
    /// Angle of rotation around Z axis in range [-pi, pi] (_roll_).
    pub z: T,
}

impl<T: Clone> From<[T; 3]> for TaitBryanAngles<T> {
    fn from(v: [T; 3]) -> Self {
        TaitBryanAngles {
            x: v[0].clone(),
            y: v[1].clone(),
            z: v[2].clone(),
        }
    }
}

impl<T> Into<[T; 3]> for TaitBryanAngles<T> {
    fn into(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}


/// Proper Euler rotation angles in ZXZ convention, that is:
///   - first, rotation around Z axis is done
///   - next, rotation around the original X axis is done
///   - finally, rotation around the original Z axis is done again
#[repr(C)]
pub struct ProperEulerAngles<T> {
    /// Angle of rotation around Z axis in range [-pi, pi].
    pub alpha: T,
    /// Angle of rotation around X axis in range [-pi/2, pi/2].
    pub beta: T,
    /// Angle of rotation around Z axis in range [-pi, pi].
    pub gamma: T,
}

impl<T: Clone> From<[T; 3]> for ProperEulerAngles<T> {
    fn from(v: [T; 3]) -> Self {
        ProperEulerAngles {
            alpha: v[0].clone(),
            beta: v[1].clone(),
            gamma: v[2].clone(),
        }
    }
}

impl<T> Into<[T; 3]> for ProperEulerAngles<T> {
    fn into(self) -> [T; 3] {
        [self.alpha, self.beta, self.gamma]
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
