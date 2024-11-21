use center::Center;
use container::Container;
use visora_core::widget::Render;
use text::{RichText, Text, Vlist};

pub mod text;
pub mod center;
pub mod container;
pub mod list;
pub mod button;
pub mod image;
macro_rules! trim_plus {
    (+ $($rest:tt)*) => {
        $($rest:tt)*
    };
}
#[macro_export]
macro_rules! internal_impl {
    ($($traits:ident$(<$slf:ident>)?),*; $($rules:tt)*) => {
        macro_rules! __emit__{ $($rules)*}
        __emit__!{
            $(
                Render<$traits$(<$slf>)?> +
            )*
            Send + Sync
        }
    }
}

/*internal_impl!{Text; ( $($bounds:tt)* ) => (
    pub trait All : $($bounds)* {}
    impl<T: $($bounds)*> All for T{}
)}*/
/*
pub trait All: Render<Text> + Render<RichText> + Render<Container<Self>> + Render<Center<Self>> + Render<Vlist<Self>>
{}
impl<R: Render<Text> +

*/
