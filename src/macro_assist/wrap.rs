pub struct Wrap<T>(pub T);

#[macro_export]
macro_rules! wrap_default {
    ($t:ty) => {
        $crate::macro_assist::Wrap(<$t>::default())
    }
}