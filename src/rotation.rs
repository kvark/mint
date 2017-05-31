use vector::Vector3;


/// Euler rotation angles in 3D space.
pub struct EulerAngles<T> {
    /// Angle between the x axis and the N axis, also known as "phi".
    pub a: T,
    /// Angle between the z axis and the Z axis, also known as "theta".
    pub b: T,
    /// Angle between the N axis and the X axis, also known as "psi".
    pub y: T,
}

impl<T: Clone> From<[T; 3]> for EulerAngles<T> {
    fn from(v: [T; 3]) -> Self {
        EulerAngles {
            a: v[0].clone(),
            b: v[1].clone(),
            y: v[2].clone(),
        }
    }
}

impl<T> Into<[T; 3]> for EulerAngles<T> {
    fn into(self) -> [T; 3] {
        [self.a, self.b, self.y]
    }
}


/// Standard quaternion represented by the scalar and vector parts.
/// Useful for representing rotation in 3D space.
pub struct Quaternion<T> {
    /// Scalar part of a quaternion.
    pub r: T,
    /// Vector part of a quaternion.
    pub v: Vector3<T>,
}

impl<T: Clone> From<[T; 4]> for Quaternion<T> {
    fn from(v: [T; 4]) -> Self {
        Quaternion {
            r: v[3].clone(),
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
        [self.v.x, self.v.y, self.v.z, self.r]
    }
}
