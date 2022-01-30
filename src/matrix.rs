use crate::vector::{Vector2, Vector3, Vector4};
use crate::IntoMint;

macro_rules! matrix {
    ($name:ident : $vec:ident[ $($field:ident[$($sub:ident),*] = $index:expr),* ] = ($inner:expr, $outer:expr)) => {
        #[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
        #[repr(C)]
        #[allow(missing_docs)] //TODO: actually have docs
        pub struct $name<T> {
            $( pub $field : $vec<T>, )*
        }

        impl<T> IntoMint for $name<T> {
            type MintType = $name<T>;
        }

        impl<T> From<[[T; $inner]; $outer]> for $name<T> {
            fn from([$($field),*]: [[T; $inner]; $outer]) -> Self {
                $name {
                    $(
                        $field: From::from($field),
                    )*
                }
            }
        }

        impl<T> From<$name<T>> for [[T; $inner]; $outer] {
            fn from(name: $name<T>) -> [[T; $inner]; $outer] {
                [$( name.$field.into() ),*]
            }
        }

        impl<T> AsRef<[[T; $inner]; $outer]> for $name<T> {
            fn as_ref(&self) -> &[[T; $inner]; $outer] { unsafe { ::core::mem::transmute(self) } }
        }

        impl<T> AsMut<[[T; $inner]; $outer]> for $name<T> {
            fn as_mut(&mut self) -> &mut [[T; $inner]; $outer] { unsafe { ::core::mem::transmute(self) } }
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

        impl<T> From<$name<T>> for [T; $inner * $outer] {
            fn from(name: $name<T>) -> [T; $inner * $outer] {
                let $name { $($field),* } = name;
                [
                    $( $( $field.$sub ),* ),*
                ]
            }
        }

        impl<T> AsRef<[T; $inner * $outer]> for $name<T> {
            fn as_ref(&self) -> &[T; $inner * $outer] { unsafe { ::core::mem::transmute(self) } }
        }

        impl<T> AsMut<[T; $inner * $outer]> for $name<T> {
            fn as_mut(&mut self) -> &mut [T; $inner * $outer] { unsafe { ::core::mem::transmute(self) } }
        }

        #[cfg(feature = "serde")]
        impl<T> ::serde::Serialize for $name<T>
            where T: ::serde::Serialize
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer
            {
                AsRef::<[[T; $inner]; $outer]>::as_ref(self).serialize(serializer)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de, T> ::serde::Deserialize<'de> for $name<T>
            where T: ::serde::Deserialize<'de>
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer<'de>
            {
                <[[T; $inner]; $outer]>::deserialize(deserializer).map($name::<T>::from)
            }
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

// 2x2 row-major matrix.
matrix!( RowMatrix2 : Vector2[x[x,y]=0,y[x,y]=1] = (2, 2));
turn!( RowMatrix2 : Vector2[x[x,y],y[x,y]] = ColumnMatrix2 );
// 2x3 row-major matrix.
// Useful for combining rotation, scale, and translation in 2D space.
matrix!( RowMatrix2x3 : Vector3[x[x,y,z]=0,y[x,y,z]=1] = (3, 2));
turn!( RowMatrix2x3 : Vector3[x[x,y,z],y[x,y,z]] = ColumnMatrix2x3 );
// 2x4 row-major matrix.
matrix!( RowMatrix2x4 : Vector4[x[x,y,z,w]=0,y[x,y,z,w]=1] = (4, 2));
turn!( RowMatrix2x4 : Vector4[x[x,y,z,w],y[x,y,z,w]] = ColumnMatrix2x4 );
// 3x2 row-major matrix.
// Useful for combining rotation, scale, and translation in 2D space.
matrix!( RowMatrix3x2 : Vector2[x[x,y]=0,y[x,y]=1,z[x,y]=2] = (2, 3));
turn!( RowMatrix3x2 : Vector2[x[x,y],y[x,y],z[x,y]] = ColumnMatrix3x2 );
// 3x3 row-major matrix.
// Useful for representing rotation and scale in 3D space.
matrix!( RowMatrix3 : Vector3[x[x,y,z]=0,y[x,y,z]=1,z[x,y,z]=2] = (3, 3));
turn!( RowMatrix3 : Vector3[x[x,y,z],y[x,y,z],z[x,y,z]] = ColumnMatrix3 );
// 3x4 row-major matrix.
// Useful for combining rotation, scale, and translation in 3D space.
matrix!( RowMatrix3x4 : Vector4[x[x,y,z,w]=0,y[x,y,z,w]=1,z[x,y,z,w]=2] = (4, 3));
turn!( RowMatrix3x4 : Vector4[x[x,y,z,w],y[x,y,z,w],z[x,y,z,w]] = ColumnMatrix3x4 );
// 4x3 row-major matrix.
// Useful for combining rotation, scale, and translation in 3D space.
matrix!( RowMatrix4x3 : Vector3[x[x,y,z]=0,y[x,y,z]=1,z[x,y,z]=2,w[x,y,z]=3] = (3, 4));
turn!( RowMatrix4x3 : Vector3[x[x,y,z],y[x,y,z],z[x,y,z],w[x,y,z]] = ColumnMatrix4x3 );
// 4x2 row-major matrix.
matrix!( RowMatrix4x2 : Vector2[x[x,y]=0,y[x,y]=1,z[x,y]=2,w[x,y]=3] = (2, 4));
turn!( RowMatrix4x2 : Vector2[x[x,y],y[x,y],z[x,y],w[x,y]] = ColumnMatrix4x2 );
// 4x4 row-major matrix.
matrix!( RowMatrix4 : Vector4[x[x,y,z,w]=0,y[x,y,z,w]=1,z[x,y,z,w]=2,w[x,y,z,w]=3] = (4, 4));
turn!( RowMatrix4 : Vector4[x[x,y,z,w],y[x,y,z,w],z[x,y,z,w],w[x,y,z,w]] = ColumnMatrix4 );

// 2x2 column-major matrix.
matrix!( ColumnMatrix2 : Vector2[x[x,y]=0,y[x,y]=1] = (2, 2));
turn!( ColumnMatrix2 : Vector2[x[x,y],y[x,y]] = RowMatrix2 );
// 2x3 column-major matrix.
// Useful for combining rotation, scale, and translation in 2D space.
matrix!( ColumnMatrix2x3 : Vector2[x[x,y]=0,y[x,y]=1,z[x,y]=2] = (2, 3));
turn!( ColumnMatrix2x3 : Vector2[x[x,y],y[x,y],z[x,y]] = RowMatrix2x3 );
// 2x4 column-major matrix.
matrix!( ColumnMatrix2x4 : Vector2[x[x,y]=0,y[x,y]=1,z[x,y]=2,w[x,y]=3] = (2, 4));
turn!( ColumnMatrix2x4 : Vector2[x[x,y],y[x,y],z[x,y],w[x,y]] = RowMatrix2x4 );
// 3x2 column-major matrix.
// Useful for combining rotation, scale, and translation in 2D space.
matrix!( ColumnMatrix3x2 : Vector3[x[x,y,z]=0,y[x,y,z]=1] = (3, 2));
turn!( ColumnMatrix3x2 : Vector3[x[x,y,z],y[x,y,z]] = RowMatrix3x2 );
// 3x3 column-major matrix.
// Useful for representing rotation and scale in 3D space.
matrix!( ColumnMatrix3 : Vector3[x[x,y,z]=0,y[x,y,z]=1,z[x,y,z]=2] = (3, 3));
turn!( ColumnMatrix3 : Vector3[x[x,y,z],y[x,y,z],z[x,y,z]] = RowMatrix3 );
// 3x4 column-major matrix.
// Useful for combining rotation, scale, and translation in 3D space.
matrix!( ColumnMatrix3x4 : Vector3[x[x,y,z]=0,y[x,y,z]=1,z[x,y,z]=2,w[x,y,z]=3] = (3, 4));
turn!( ColumnMatrix3x4 : Vector3[x[x,y,z],y[x,y,z],z[x,y,z],w[x,y,z]] = RowMatrix3x4 );
// 4x2 column-major matrix.
matrix!( ColumnMatrix4x2 : Vector4[x[x,y,z,w]=0,y[x,y,z,w]=1] = (4, 2));
turn!( ColumnMatrix4x2 : Vector4[x[x,y,z,w],y[x,y,z,w]] = RowMatrix4x2 );
// 4x3 column-major matrix.
// Useful for combining rotation, scale, and translation in 3D space.
matrix!( ColumnMatrix4x3 : Vector4[x[x,y,z,w]=0,y[x,y,z,w]=1,z[x,y,z,w]=2] = (4, 3));
turn!( ColumnMatrix4x3 : Vector4[x[x,y,z,w],y[x,y,z,w],z[x,y,z,w]] = RowMatrix4x3 );
// 4x4 column-major matrix.
matrix!( ColumnMatrix4 : Vector4[x[x,y,z,w]=0,y[x,y,z,w]=1,z[x,y,z,w]=2,w[x,y,z,w]=3] = (4, 4));
turn!( ColumnMatrix4 : Vector4[x[x,y,z,w],y[x,y,z,w],z[x,y,z,w],w[x,y,z,w]] = RowMatrix4 );
