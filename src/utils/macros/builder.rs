#[macro_export]
macro_rules! build_fn {
    ( $vis:vis $name:ident $( $field:ident ),+ ) => {
        #[doc = concat!("Constructs a new [`", stringify!($name),"`] taking all fields values from this object.")]
        $vis fn build(self) -> $name {
            $name {
                $(
                    $field: self.$field,
                )*
            }
        }
    };
}
pub use build_fn;

#[macro_export]
macro_rules! new {
    ( $vis:vis $name:ident ) => {
        #[doc = concat!("Constructs a new [`", stringify!($name),"`].")]
        $vis fn new() -> $name {
            $name {
                ..Default::default()
            }
        }
    };
}
pub use new;

#[macro_export]
macro_rules! from {
    ( $name:ident $to:ident $( $field:ident ),+ ) => {
        impl From<$name> for $to {
            fn from(value: $name) -> Self {
                Self {
                    $( $field: value.$field, )*
                }
            }
        }

        impl From<$to> for $name {
            fn from(value: $to) -> Self {
                value.build()
            }
        }
    };
}
pub use from;

#[macro_export]
/// Builds a new builder with chainable configuration methods to create a new object.
///
/// ## Example
///
/// ```rust
/// use tosic_utils::utils::builder::builder;
///
/// builder! {
///     MyObjectBuilder;
///
///     #[derive(Default)]
///     pub struct MyObject {
///         id: Option<i32>,
///         name: String,
///         age: i32
///     }
/// }
///
/// impl MyObjectBuilder {
///     pub fn id(mut self, id: i32) -> Self {
///         self.id = Some(id);
///         self
///     }
///
///     pub fn name(mut self, name: String) -> Self {
///         self.name = name;
///         self
///     }
///
///     pub fn age(mut self, age: i32) -> Self {
///         self.age = age;
///         self
///     }
/// }
/// ```
macro_rules! builder {
    ( $( #[$builder_meta:meta] )* $builder_name:ident; $(#[$meta:meta])* $vis:vis $key:ident $name:ident $( $tt:tt )* ) => {
        builder!( @type_impl $( #[$meta] )* $vis $key $name $( $tt )* );
        builder!( @builder_impl $( #[$builder_meta] )* $builder_name $( #[$meta] )* $vis $key $name $( $tt )* );
    };

    ( @type_impl $( #[$meta:meta] )* $vis:vis $key:ident $name:ident
        { $( $( #[$field_meta:meta] )* $field_vis:vis $field:ident: $field_ty:ty, )* }
    ) => {

        $( #[$meta] )*
        $vis $key $name {
            $( $( #[$field_meta] )* $field_vis $field: $field_ty, )*
        }
    };

    ( @builder_impl $( #[$builder_meta:meta] )* $builder_name:ident $( #[$meta:meta] )* $vis:vis $key:ident $name:ident
        { $( $( #[$field_meta:meta] )* $field_vis:vis $field:ident: $field_ty:ty, )* }
    ) => {
        #[doc = concat!("Builder for [`", stringify!($name),
            "`] with chainable configuration methods to create a new [`", stringify!($name) , "`].")]
        $( #[$builder_meta] )*
        $vis $key $builder_name {
            $( $field: $field_ty, )*
        }

        impl Default for $builder_name {
            fn default() -> Self {
                let meta_default: $name = $name::default();
                Self {
                    $( $field: meta_default.$field, )*
                }
            }
        }

        impl $builder_name {
            tosic_utils::utils::macros::builder::new!($vis $builder_name);
            tosic_utils::utils::macros::builder::build_fn!($vis $name $( $field ),* );
        }

        tosic_utils::utils::macros::builder::from!($name $builder_name $( $field ),* );
    };
}
pub use builder;
