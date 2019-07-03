macro_rules! vec {
    ($name:ident [ $($field:ident),* ] = $fixed:ty) => {
        #[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
        #[repr(C)]
        #[allow(missing_docs)] //TODO: actually have docs
        pub struct $name<T> {
            $( pub $field : T, )*
        }

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
            #[allow(missing_docs)]
            pub fn from_slice(slice: &[T]) -> Self {
                let mut iter = slice.iter();
                $name {
                    $(
                        $field: iter
                            .next()
                            .expect(&concat!("Missing ", stringify!($field), "-axis in slice."))
                            .clone()
                    ),*
                }
            }
        }
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

vec!( Vector2 [x, y] = [T; 2] );
from!( Vector2 [x,y] = Point2 );
vec!( Vector3 [x, y, z] = [T; 3] );
from!( Vector3 [x,y,z] = Point3 );
vec!( Vector4 [x, y, z, w] = [T; 4] );
vec!( Point2 [x, y] = [T; 2] );
from!( Point2 [x,y] = Vector2 );
vec!( Point3 [x, y, z] = [T; 3] );
from!( Point3 [x,y,z] = Vector3 );
vec!( Bivector [xy=0, xz=1, yz=2] = [T; 3] );
