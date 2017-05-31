/// Vector in 2D space.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct Vector2<T> {
    /// X component of a vector.
    pub x: T,
    /// Y component of a vector.
    pub y: T,
}

impl<T: Clone> From<[T; 2]> for Vector2<T> {
    fn from(v: [T; 2]) -> Self {
        Vector2 {
            x: v[0].clone(),
            y: v[1].clone(),
        }
    }
}

impl<T> Into<[T; 2]> for Vector2<T> {
    fn into(self) -> [T; 2] {
        [self.x, self.y]
    }
}


/// Vector in 3D space.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct Vector3<T> {
    /// X component of a vector.
    pub x: T,
    /// Y component of a vector.
    pub y: T,
    /// Z component of a vector.
    pub z: T,
}

impl<T: Clone> From<[T; 3]> for Vector3<T> {
    fn from(v: [T; 3]) -> Self {
        Vector3 {
            x: v[0].clone(),
            y: v[1].clone(),
            z: v[2].clone(),
        }
    }
}

impl<T> Into<[T; 3]> for Vector3<T> {
    fn into(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}


/// Vector in 4D space.
/// Useful as a homogeneous 3D vector representation.
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[repr(C)]
pub struct Vector4<T> {
    /// X component of a vector.
    pub x: T,
    /// Y component of a vector.
    pub y: T,
    /// Z component of a vector.
    pub z: T,
    /// W component of a vector.
    pub w: T,
}

impl<T: Clone> From<[T; 4]> for Vector4<T> {
    fn from(v: [T; 4]) -> Self {
        Vector4 {
            x: v[0].clone(),
            y: v[1].clone(),
            z: v[2].clone(),
            w: v[3].clone(),
        }
    }
}

impl<T> Into<[T; 4]> for Vector4<T> {
    fn into(self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }
}
