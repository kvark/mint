use vector::{Vector2, Vector3, Vector4};
use std::mem;

macro_rules! decl_matrix_type {
    (
        $(#[$attr:meta])*
        $v:vis struct $name:ident {
            $(
                #[doc = $field_doc:expr]
                $field_vis:vis $field:ident: $field_ty:ident
            ),*
        }
    ) => {
        $(#[$attr])*
        $v struct $name<T> {
            $(
                #[doc = $field_doc]
                $field_vis $field: $field_ty<T>
            ),*
        }
    };
}

macro_rules! matrix {
    (
        $( #[doc = $doc:expr] )*
        $name:ident: $major_ty:ident $vec:ident[ $($field:ident[$($sub:ident),*] = $index:expr),* ] = ($inner:expr, $outer:expr)
    ) => {
        decl_matrix_type! {
            $( #[doc = $doc] )*
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
            pub struct $name {
                $(
                    #[doc = concat!(
                        stringify!($major_ty), " ", stringify!($index), " of the matrix.",
                    )]
                    pub $field: $vec
                ),*
            }
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

        impl<T> Into<[T; $inner * $outer]> for $name<T> {
            fn into(self) -> [T; $inner * $outer] {
                let $name { $($field),* } = self;
                [
                    $( $( $field.$sub ),* ),*
                ]
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

matrix!(
    #[doc = "A 2×2 row-major matrix."]
    RowMatrix2 : Row Vector2[x[x,y]=0,y[x,y]=1] = (2, 2)
);
turn!( RowMatrix2 : Vector2[x[x,y],y[x,y]] = ColumnMatrix2 );

matrix!(
    #[doc = "A 2×3 row-major matrix."]
    #[doc = "\n"]
    #[doc = "Useful for combining rotation, scale, and translation in 2D space."]
    RowMatrix2x3 : Row Vector3[x[x,y,z]=0,y[x,y,z]=1] = (3, 2)
);
turn!( RowMatrix2x3 : Vector3[x[x,y,z],y[x,y,z]] = ColumnMatrix2x3 );

matrix!(
    #[doc = "A 3×2 row-major matrix."]
    #[doc = "\n"]
    #[doc = "Useful for combining rotation, scale, and translation in 2D space."]
    RowMatrix3x2 : Row Vector2[x[x,y]=0,y[x,y]=1,z[x,y]=2] = (2, 3)
);
turn!( RowMatrix3x2 : Vector2[x[x,y],y[x,y],z[x,y]] = ColumnMatrix3x2 );

matrix!(
    #[doc = "A 3×3 row-major matrix."]
    #[doc = "\n"]
    #[doc = "Useful for representing rotation and scale in 3D space."]
    RowMatrix3 : Row Vector3[x[x,y,z]=0,y[x,y,z]=1,z[x,y,z]=2] = (3, 3)
);
turn!( RowMatrix3 : Vector3[x[x,y,z],y[x,y,z],z[x,y,z]] = ColumnMatrix3 );

matrix!(
    #[doc = "A 3×4 row-major matrix."]
    #[doc = "\n"]
    #[doc = "Useful for combining rotation, scale, and translation in 3D space."]
    RowMatrix3x4 : Row Vector4[x[x,y,z,w]=0,y[x,y,z,w]=1,z[x,y,z,w]=2] = (4, 3)
);
turn!( RowMatrix3x4 : Vector4[x[x,y,z,w],y[x,y,z,w],z[x,y,z,w]] = ColumnMatrix3x4 );

matrix!(
    #[doc = "A 4×3 row-major matrix."]
    #[doc = "\n"]
    #[doc = "Useful for combining rotation, scale, and translation in 3D space."]
    RowMatrix4x3 : Row Vector3[x[x,y,z]=0,y[x,y,z]=1,z[x,y,z]=2,w[x,y,z]=3] = (3, 4)
);
turn!( RowMatrix4x3 : Vector3[x[x,y,z],y[x,y,z],z[x,y,z],w[x,y,z]] = ColumnMatrix4x3 );

matrix!(
    #[doc = "A 4×4 row-major matrix."]
    RowMatrix4 : Row Vector4[x[x,y,z,w]=0,y[x,y,z,w]=1,z[x,y,z,w]=2,w[x,y,z,w]=3] = (4, 4)
);
turn!( RowMatrix4 : Vector4[x[x,y,z,w],y[x,y,z,w],z[x,y,z,w],w[x,y,z,w]] = ColumnMatrix4 );


matrix!(
    #[doc = "A 2×2 column-major matrix."]
    ColumnMatrix2 : Column Vector2[x[x,y]=0,y[x,y]=1] = (2, 2)
);
turn!( ColumnMatrix2 : Vector2[x[x,y],y[x,y]] = RowMatrix2 );

matrix!(
    #[doc = "A 2×3 column-major matrix."]
    #[doc = "\n"]
    #[doc = "Useful for combining rotation, scale, and translation in 2D space."]
    ColumnMatrix2x3 : Column Vector2[x[x,y]=0,y[x,y]=1,z[x,y]=2] = (2, 3)
);
turn!( ColumnMatrix2x3 : Vector2[x[x,y],y[x,y],z[x,y]] = RowMatrix2x3 );

matrix!(
    #[doc = "A 3×2 column-major matrix."]
    #[doc = "\n"]
    #[doc = "Useful for combining rotation, scale, and translation in 2D space."]
    ColumnMatrix3x2 : Column Vector3[x[x,y,z]=0,y[x,y,z]=1] = (3, 2)
);
turn!( ColumnMatrix3x2 : Vector3[x[x,y,z],y[x,y,z]] = RowMatrix3x2 );

matrix!(
    #[doc = "A 3×3 column-major matrix."]
    #[doc = "\n"]
    #[doc = "Useful for representing rotation and scale in 3D space."]
    ColumnMatrix3 : Column Vector3[x[x,y,z]=0,y[x,y,z]=1,z[x,y,z]=2] = (3, 3)
);
turn!( ColumnMatrix3 : Vector3[x[x,y,z],y[x,y,z],z[x,y,z]] = RowMatrix3 );

matrix!(
    #[doc = "A 3×4 column-major matrix."]
    #[doc = "\n"]
    #[doc = "Useful for combining rotation, scale, and translation in 3D space."]
    ColumnMatrix3x4 : Column Vector3[x[x,y,z]=0,y[x,y,z]=1,z[x,y,z]=2,w[x,y,z]=3] = (3, 4)
);
turn!( ColumnMatrix3x4 : Vector3[x[x,y,z],y[x,y,z],z[x,y,z],w[x,y,z]] = RowMatrix3x4 );

matrix!(
    #[doc = "A 4×3 column-major matrix."]
    #[doc = "\n"]
    #[doc = "Useful for combining rotation, scale, and translation in 3D space."]
    ColumnMatrix4x3 : Column Vector4[x[x,y,z,w]=0,y[x,y,z,w]=1,z[x,y,z,w]=2] = (4, 3)
);
turn!( ColumnMatrix4x3 : Vector4[x[x,y,z,w],y[x,y,z,w],z[x,y,z,w]] = RowMatrix4x3 );

matrix!(
    #[doc = "A 4×4 column-major matrix."]
    ColumnMatrix4 : Column Vector4[x[x,y,z,w]=0,y[x,y,z,w]=1,z[x,y,z,w]=2,w[x,y,z,w]=3] = (4, 4)
);
turn!( ColumnMatrix4 : Vector4[x[x,y,z,w],y[x,y,z,w],z[x,y,z,w],w[x,y,z,w]] = RowMatrix4 );
