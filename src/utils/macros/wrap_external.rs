#[macro_export]
macro_rules! wrap_external_type {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident($wrapped:path);

        $(
            impl $($impl_code:tt)*
        )*
    ) => {
        $(#[$meta])*
        #[doc("Wraps a type in the [`", stringify!($wrapped), "`] type.")]
        $vis struct $name($wrapped);

        impl std::ops::Deref for $name {
            type Target = $wrapped;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<$wrapped> for $name {
            fn from(val: $wrapped) -> Self {
                Self(val)
            }
        }

        impl From<$name> for $wrapped {
            fn from(val: $name) -> Self {
                val.0
            }
        }

        $(
            impl $($impl_code)*
        )*
    };
}

pub use wrap_external_type;
