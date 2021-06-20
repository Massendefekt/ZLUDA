macro_rules! sub_enum {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        sub_enum!{ $name : ScalarType { $($variant),+ } }
    };
    ($name:ident : $base_type:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(PartialEq, Eq, Clone, Copy)]
        pub enum $name {
            $(
                $variant,
            )+
        }

        impl From<$name> for $base_type {
            fn from(t: $name) -> $base_type {
                match t {
                    $(
                        $name::$variant => $base_type::$variant,
                    )+
                }
            }
        }

        impl std::convert::TryFrom<$base_type> for $name {
            type Error = ();

            fn try_from(t: $base_type) -> Result<Self, Self::Error> {
                match t {
                    $(
                        $base_type::$variant => Ok($name::$variant),
                    )+
                        _ => Err(()),
                }
            }
        }
    };
}

macro_rules! sub_type {
    ($type_name:ident { $($variant:ident ( $($field_type:ident),+ ) ),+ $(,)? } ) => {
        sub_type! { $type_name : Type {
            $(
                $variant ($($field_type),+),
            )+
         }}
    };
    ($type_name:ident : $base_type:ident { $($variant:ident ( $($field_type:ident),+ ) ),+ $(,)? } ) => {
        #[derive(PartialEq, Eq, Clone)]
        pub enum $type_name {
            $(
                $variant ($($field_type),+),
            )+
        }

        impl From<$type_name> for $base_type {
            #[allow(non_snake_case)]
            fn from(t: $type_name) -> $base_type {
                match t {
                    $(
                        $type_name::$variant ( $($field_type),+ ) => <$base_type>::$variant ( $($field_type.into()),+),
                    )+
                }
            }
        }

        impl std::convert::TryFrom<$base_type> for $type_name {
            type Error = ();

            #[allow(non_snake_case)]
            #[allow(unreachable_patterns)]
            fn try_from(t: $base_type) -> Result<Self, Self::Error> {
                match t {
                    $(
                        $base_type::$variant ( $($field_type),+ )  => Ok($type_name::$variant ( $($field_type.try_into().map_err(|_| ())? ),+ )),
                    )+
                        _ => Err(()),
                }
            }
        }
    };
}
