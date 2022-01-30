use crate::IntoMint;

macro_rules! vec {
    ($name:ident [ $($field:ident),* ] = $fixed:ty) => {
        #[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
        #[repr(C)]
        #[allow(missing_docs)] //TODO: actually have docs
        pub struct $name<T> {
            $( pub $field : T, )*
        }

        impl<T> IntoMint for $name<T> {
            type MintType = $name<T>;
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

        impl<T> From<$name<T>> for $fixed {
            fn from(name: $name<T>) -> $fixed {
                [$(name.$field.into() ),*]
            }
        }

        impl<T> AsRef<$fixed> for $name<T> {
            fn as_ref(&self) -> &$fixed { unsafe { ::core::mem::transmute(self) } }
        }

        impl<T> AsMut<$fixed> for $name<T> {
            fn as_mut(&mut self) -> &mut $fixed { unsafe { ::core::mem::transmute(self) } }
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

        #[cfg(feature = "serde")]
        impl<T> ::serde::Serialize for $name<T>
            where T: ::serde::Serialize
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer
            {
                AsRef::<$fixed>::as_ref(self).serialize(serializer)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de, T> ::serde::Deserialize<'de> for $name<T>
            where T: ::serde::Deserialize<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer<'de>
            {
                <$fixed>::deserialize(deserializer).map($name::<T>::from)
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
