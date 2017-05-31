use vector::{Vector2, Vector3, Vector4};

macro_rules! matrix {
    ($name:ident : $vec:ident[ $($field:ident = $index:expr),* ] = $fixed:ty) => {
        #[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
        #[allow(missing_docs)] //TODO: actually have docs
        pub struct $name<T> {
            $( pub $field : $vec<T>, )*
        }

        //TODO: revise the `Copy` bound here
        impl<T: Copy> From<$fixed> for $name<T> {
            fn from(m: $fixed) -> Self {
                $name {
                    $(
                        $field: m[$index].into(),
                    )*
                }
            }
        }

        impl<T> Into<$fixed> for $name<T> {
            fn into(self) -> $fixed {
                [$( self.$field.into() ),*]
            }
        }
    }
}

/// 2x2 row-major matrix.
matrix!( RowMatrix2 : Vector2[x=0,y=1] = [[T; 2]; 2]);
/// 2x3 row-major matrix.
/// Useful for combinging rotation, scale, and translation in 2D space.
matrix!( RowMatrix2x3 : Vector2[x=0,y=1,z=2] = [[T; 2]; 3]);
/// 3x3 row-major matrix.
/// Useful for representing rotation and scale in 3D space.
matrix!( RowMatrix3 : Vector3[x=0,y=1,z=2] = [[T; 3]; 3]);
/// 3x4 row-major matrix.
/// Useful for combinging rotation, scale, and translation in 3D space.
matrix!( RowMatrix3x4 : Vector3[x=0,y=1,z=2,w=3] = [[T; 3]; 4]);
/// 4x4 row-major matrix.
matrix!( RowMatrix4 : Vector4[x=0,y=1,z=2,w=3] = [[T; 4]; 4]);

/// 2x2 column-major matrix.
matrix!( ColumnMatrix2 : Vector2[x=0,y=1] = [[T; 2]; 2]);
/// 3x2 column-major matrix.
/// Useful for combinging rotation, scale, and translation in 2D space.
matrix!( ColumnMatrix3x2 : Vector3[x=0,y=1] = [[T; 3]; 2]);
/// 3x3 column-major matrix.
/// Useful for representing rotation and scale in 3D space.
matrix!( ColumnMatrix3 : Vector3[x=0,y=1,z=2] = [[T; 3]; 3]);
/// 4x3 column-major matrix.
/// Useful for combinging rotation, scale, and translation in 3D space.
matrix!( ColumnMatrix4x3 : Vector4[x=0,y=1,z=2] = [[T; 4]; 3]);
/// 4x4 column-major matrix.
matrix!( ColumnMatrix4 : Vector4[x=0,y=1,z=2,w=3] = [[T; 4]; 4]);
