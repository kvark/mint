use vector::{Vector2, Vector3, Vector4};

pub use future_self::{
    ColumnMatrix2, ColumnMatrix3, ColumnMatrix4,
    RowMatrix2, RowMatrix3, RowMatrix4,
};

macro_rules! matrix {
    ($name:ident : $vec:ident[ $($field:ident = $index:expr),* ] = $fixed:ty) => {
        #[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
        #[repr(C)]
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

macro_rules! transpose {
    ($name:ident : $vec:ident[$( $field:ident [ $($sub:ident),* ]  ),* ] = $transposed:ident) => {
        impl<T> From<$transposed<T>> for $name<T> {
            fn from(m: $transposed<T>) -> Self {
                $name {
                    $(
                        $field: $vec {
                            $(
                                $sub: m.$sub.$field,
                            )*
                        },
                    )*
                }
            }
        }
    }
}

/// 2x3 row-major matrix.
/// Useful for combinging rotation, scale, and translation in 2D space.
matrix!( RowMatrix2x3 : Vector2[x=0,y=1,z=2] = [[T; 2]; 3]);
transpose!( RowMatrix2x3 : Vector2[x[x,y],y[x,y],z[x,y]] = ColumnMatrix3x2 );
/// 3x4 row-major matrix.
/// Useful for combinging rotation, scale, and translation in 3D space.
matrix!( RowMatrix3x4 : Vector3[x=0,y=1,z=2,w=3] = [[T; 3]; 4]);
transpose!( RowMatrix3x4 : Vector3[x[x,y,z],y[x,y,z],z[x,y,z],w[x,y,z]] = ColumnMatrix4x3 );

/// 3x2 column-major matrix.
/// Useful for combinging rotation, scale, and translation in 2D space.
matrix!( ColumnMatrix3x2 : Vector3[x=0,y=1] = [[T; 3]; 2]);
transpose!( ColumnMatrix3x2 : Vector3[x[x,y,z],y[x,y,z]] = RowMatrix2x3 );
/// 4x3 column-major matrix.
/// Useful for combinging rotation, scale, and translation in 3D space.
matrix!( ColumnMatrix4x3 : Vector4[x=0,y=1,z=2] = [[T; 4]; 3]);
transpose!( ColumnMatrix4x3 : Vector4[x[x,y,z,w],y[x,y,z,w],z[x,y,z,w]] = RowMatrix3x4 );
