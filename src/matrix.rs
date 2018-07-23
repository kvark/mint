use vector::{Vector2, Vector3, Vector4};
use std::{mem, ptr};

macro_rules! matrix {
    ($name:ident : $vec:ident[ $($field:ident = $index:expr),* ] = ($inner:expr, $outer:expr)) => {
        #[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
        #[repr(C)]
        #[allow(missing_docs)] //TODO: actually have docs
        pub struct $name<T> {
            $( pub $field : $vec<T>, )*
        }

        //TODO: revise the `Copy` bound here
        impl<T: Copy> From<[[T; $inner]; $outer]> for $name<T> {
            fn from(m: [[T; $inner]; $outer]) -> Self {
                $name {
                    $(
                        $field: m[$index].into(),
                    )*
                }
            }
        }

        impl<T> Into<[[T; $inner]; $outer]> for $name<T> {
            fn into(self) -> [[T; $inner]; $outer] {
                [$( self.$field.into() ),*]
            }
        }

        impl<T> AsRef<[[T; $inner]; $outer]> for $name<T> {
            fn as_ref(&self) -> &[[T; $inner]; $outer] { unsafe { mem::transmute(self) } }
        }

        impl<T: Clone> From<[T; $inner * $outer]> for $name<T> {
            fn from(m: [T; $inner * $outer]) -> Self {
                $name {
                    $(
                        $field: $vec::from_slice(&m[$index*$inner..($index+1)*$inner]),
                    )*
                }
            }
        }

        //TODO: revise the `Copy` bound here
        impl<T: Copy> Into<[T; $inner * $outer]> for $name<T> {
            fn into(self) -> [T; $inner * $outer] {
                unsafe {
                    let mut out: [T; $inner * $outer] = mem::uninitialized();
                    $(
                        ptr::copy_nonoverlapping(self.$field.as_ref().as_ptr(), out[$index*$inner..($index+1)*$inner].as_mut_ptr(), $inner);
                    )*
                    out
                }
            }
        }

        impl<T> AsRef<[T; $inner * $outer]> for $name<T> {
            fn as_ref(&self) -> &[T; $inner * $outer] { unsafe { mem::transmute(self) } }
        }
    };
}

macro_rules! turn {
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

/// 2x2 row-major matrix.
matrix!( RowMatrix2 : Vector2[x=0,y=1] = (2, 2));
turn!( RowMatrix2 : Vector2[x[x,y],y[x,y]] = ColumnMatrix2 );
/// 2x3 row-major matrix.
/// Useful for combining rotation, scale, and translation in 2D space.
matrix!( RowMatrix2x3 : Vector3[x=0,y=1] = (3, 2));
turn!( RowMatrix2x3 : Vector3[x[x,y,z],y[x,y,z]] = ColumnMatrix2x3 );
/// 3x2 row-major matrix.
/// Useful for combining rotation, scale, and translation in 2D space.
matrix!( RowMatrix3x2 : Vector2[x=0,y=1,z=2] = (2, 3));
turn!( RowMatrix3x2 : Vector2[x[x,y],y[x,y],z[x,y]] = ColumnMatrix3x2 );
/// 3x3 row-major matrix.
/// Useful for representing rotation and scale in 3D space.
matrix!( RowMatrix3 : Vector3[x=0,y=1,z=2] = (3, 3));
turn!( RowMatrix3 : Vector3[x[x,y,z],y[x,y,z],z[x,y,z]] = ColumnMatrix3 );
/// 3x4 row-major matrix.
/// Useful for combining rotation, scale, and translation in 3D space.
matrix!( RowMatrix3x4 : Vector4[x=0,y=1,z=2] = (4, 3));
turn!( RowMatrix3x4 : Vector4[x[x,y,z,w],y[x,y,z,w],z[x,y,z,w]] = ColumnMatrix3x4 );
/// 4x3 row-major matrix.
/// Useful for combining rotation, scale, and translation in 3D space.
matrix!( RowMatrix4x3 : Vector3[x=0,y=1,z=2,w=3] = (3, 4));
turn!( RowMatrix4x3 : Vector3[x[x,y,z],y[x,y,z],z[x,y,z],w[x,y,z]] = ColumnMatrix4x3 );
/// 4x4 row-major matrix.
matrix!( RowMatrix4 : Vector4[x=0,y=1,z=2,w=3] = (4, 4));
turn!( RowMatrix4 : Vector4[x[x,y,z,w],y[x,y,z,w],z[x,y,z,w],w[x,y,z,w]] = ColumnMatrix4 );

/// 2x2 column-major matrix.
matrix!( ColumnMatrix2 : Vector2[x=0,y=1] = (2, 2));
turn!( ColumnMatrix2 : Vector2[x[x,y],y[x,y]] = RowMatrix2 );
/// 2x3 column-major matrix.
/// Useful for combining rotation, scale, and translation in 2D space.
matrix!( ColumnMatrix2x3 : Vector2[x=0,y=1,z=2] = (2, 3));
turn!( ColumnMatrix2x3 : Vector2[x[x,y],y[x,y],z[x,y]] = RowMatrix2x3 );
/// 3x2 column-major matrix.
/// Useful for combining rotation, scale, and translation in 2D space.
matrix!( ColumnMatrix3x2 : Vector3[x=0,y=1] = (3, 2));
turn!( ColumnMatrix3x2 : Vector3[x[x,y,z],y[x,y,z]] = RowMatrix3x2 );
/// 3x3 column-major matrix.
/// Useful for representing rotation and scale in 3D space.
matrix!( ColumnMatrix3 : Vector3[x=0,y=1,z=2] = (3, 3));
turn!( ColumnMatrix3 : Vector3[x[x,y,z],y[x,y,z],z[x,y,z]] = RowMatrix3 );
/// 3x4 column-major matrix.
/// Useful for combining rotation, scale, and translation in 3D space.
matrix!( ColumnMatrix3x4 : Vector3[x=0,y=1,z=2,w=3] = (3, 4));
turn!( ColumnMatrix3x4 : Vector3[x[x,y,z],y[x,y,z],z[x,y,z],w[x,y,z]] = RowMatrix3x4 );
/// 4x3 column-major matrix.
/// Useful for combining rotation, scale, and translation in 3D space.
matrix!( ColumnMatrix4x3 : Vector4[x=0,y=1,z=2] = (4, 3));
turn!( ColumnMatrix4x3 : Vector4[x[x,y,z,w],y[x,y,z,w],z[x,y,z,w]] = RowMatrix4x3 );
/// 4x4 column-major matrix.
matrix!( ColumnMatrix4 : Vector4[x=0,y=1,z=2,w=3] = (4, 4));
turn!( ColumnMatrix4 : Vector4[x[x,y,z,w],y[x,y,z,w],z[x,y,z,w],w[x,y,z,w]] = RowMatrix4 );
