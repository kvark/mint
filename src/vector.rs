macro_rules! vec {
    ($name:ident [ $($field:ident = $index:expr),* ] = $fixed:ty = $len:expr) => {
        #[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
        #[repr(C)]
        #[allow(missing_docs)] //TODO: actually have docs
        pub struct $name<T> {
            $( pub $field : T, )*
        }

        impl<T: Clone> From<$fixed> for $name<T> {
            fn from(v: $fixed) -> Self {
                $name {
                    $(
                        $field: v[$index].clone(),
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
                $name {
                    $(
                        $field: slice[$index].clone(),
                    )*
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
                use ::serde::ser::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some($len))?;
                $( seq.serialize_element(&self.$field)?; )*
                seq.end()
            }
        }

        #[cfg(feature = "serde")]
        impl<'de, T> ::serde::Deserialize<'de> for $name<T>
            where T: ::serde::Deserialize<'de>
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer<'de>
            {
                struct GenericVisitor<T>(::std::marker::PhantomData<T>);
                impl<'de, T> ::serde::de::Visitor<'de> for GenericVisitor<T>
                    where T: ::serde::de::Deserialize<'de>
                {
                    type Value = $name<T>;
                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(formatter, "struct {} or {}", stringify!($name), stringify!($fixed))
                    }
                    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
                        where A: ::serde::de::SeqAccess<'de>
                    {
                        Ok($name {
                            $( $field: access.next_element::<T>()?.unwrap() ),*
                        })
                    }
                }

                deserializer.deserialize_seq(GenericVisitor::<T>(::std::marker::PhantomData))
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

vec!( Vector2 [x=0, y=1] = [T; 2] = 2);
from!( Vector2 [x,y] = Point2 );
vec!( Vector3 [x=0, y=1, z=2] = [T; 3] = 3 );
from!( Vector3 [x,y,z] = Point3 );
vec!( Vector4 [x=0, y=1, z=2, w=3] = [T; 4] = 4 );
vec!( Point2 [x=0, y=1] = [T; 2] = 2);
from!( Point2 [x,y] = Vector2 );
vec!( Point3 [x=0, y=1, z=2] = [T; 3] = 3 );
from!( Point3 [x,y,z] = Vector3 );
