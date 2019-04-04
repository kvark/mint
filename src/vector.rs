macro_rules! vec_methods {
    (doc($sname:expr) $name:ident ($dim:expr) [ $($field:ident = $index:expr),* ] = $fixed:ty) => {
        impl<T> From<$fixed> for $name<T> {
            fn from([$($field),*]: $fixed) -> Self {
                $name {
                    $(
                        $field,
                    )*
                }
            }
        }

        impl<T> Into<$fixed> for $name<T> {
            fn into(self) -> $fixed {
                [$( self.$field.into() ),*]
            }
        }

        impl<T> AsRef<$fixed> for $name<T> {
            fn as_ref(&self) -> &$fixed { unsafe { ::std::mem::transmute(self) } }
        }

        impl<T: Clone> $name<T> {
            #[doc = "Constructs a `"]
            #[doc = $sname]
            #[doc = "` from the first "]
            #[doc = $dim]
            #[doc = " elements of `slice`.\n"]
            #[doc = "# Panics\n"]
            #[doc = "This method will panic if `slice.len()` is smaller than "]
            #[doc = $dim]
            #[doc = "."]
            pub fn from_slice(slice: &[T]) -> Self {
                $name {
                    $(
                        $field: slice[$index].clone(),
                    )*
                }
            }
        }
    }
}

macro_rules! decl_vector_type {
    (
        $(#[$attr:meta])*
        $v:vis struct $name:ident<$generic:ident> {
            $(
                $(#[doc = $doc:expr])*
                $fvis:vis $field:ident : $field_ty:ident
            ),*
        }
    ) => {
        $(#[$attr])*
        $v struct $name<$generic> {
            $(
                $(#[doc = $doc])*
                $fvis $field: $field_ty
            ),*
        }
    };
}

macro_rules! vec {
    (
        $(#[doc = $doc:expr])*
        $name:ident ($dim:expr) [ $($field:ident = $index:expr),* ] = $fixed:ty
    ) => {
        decl_vector_type! {
            $(#[doc = $doc])*
            #[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
            #[repr(C)]
            pub struct $name<T> {
                $(
                    #[doc = concat!(
                        "The ", stringify!($field), " component of the `", stringify!($name), "`."
                    )]
                    pub $field: T
                ),*
            }
        }
        vec_methods!(doc(stringify!($name)) $name (stringify!($dim)) [ $($field = $index),* ] = $fixed);
    }
}

macro_rules! from {
    ($name:ident [ $($field:ident),* ] = $other:ident) => {
        impl<T> From<$other<T>> for $name<T> {
            fn from(v: $other<T>) -> Self {
                $name {
                    $(
                        $field: v.$field,
                    )*
                }
            }
        }
    }
}

vec!(
    #[doc = "A vector representing direction and magnitude in 2D space."]
    Vector2 (2) [x=0, y=1] = [T; 2]
);
from!( Vector2 [x,y] = Point2 );
vec!(
    #[doc = "A vector representing direction and magnitude in 3D space."]
    Vector3 (3) [x=0, y=1, z=2] = [T; 3]
);
from!( Vector3 [x,y,z] = Point3 );
vec!(
    #[doc = "A vector representing direction and magnitude in 4D space."]
    Vector4 (4) [x=0, y=1, z=2, w=3] = [T; 4]
);
vec!(
    #[doc = "A point representing a position in 2D space."]
    Point2 (2) [x=0, y=1] = [T; 2]
);
from!( Point2 [x,y] = Vector2 );
vec!(
    #[doc = "A point representing a position in 3D space."]
    Point3 (3) [x=0, y=1, z=2] = [T; 3]
);
from!( Point3 [x,y,z] = Vector3 );
