#[macro_export]
macro_rules! populate_impl {
    ($i:ident, $($t:ty),*) => {
        $(
            $i!($t);
        )*
    }
}
pub use populate_impl;