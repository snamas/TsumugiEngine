pub use std::any::Any;
pub use tsumugi_macro_derive::TsumugiAny;
pub use tsumugi_macro_derive::pack;
pub trait TsumugiAnyTrait {
    fn as_any(&mut self) -> &mut dyn Any;
}