#[macro_export]
/// Wrap an external type in a struct that implements `Deref` and `DerefMut`.
///
/// This can be helpful when using external types that does not impl the correct traits and needs a manual implementation.
///
/// The `Deref` and `DerefMut` traits are implemented for the wrapped type and allows for seamless usage of the external type.
///
/// # Examples
///
/// ```rust
/// use tosic_utils::wrap_external_type;
///
/// wrap_external_type! {
///     #[derive(Debug)]
///     pub struct MyExternalType(String);
/// }
/// ```
///
macro_rules! wrap_external_type {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident($wrapped:path);

        $(
            impl $($impl_code:tt)*
        )*
    ) => {
        $(#[$meta])*
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

        impl std::iter::FromIterator<$name> for Vec<$wrapped> {
            fn from_iter<T: IntoIterator<Item = $name>>(iter: T) -> Self {
                iter.into_iter().collect()
            }
        }

        $(
            impl $($impl_code)*
        )*
    };
}

pub use wrap_external_type;
