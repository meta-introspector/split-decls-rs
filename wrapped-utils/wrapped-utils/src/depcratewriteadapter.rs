// Generated macro for WriteAdapter (struct)
macro_rules! DepcrateWriteAdapter {
() => {
// Module: crate
// Provides: {"WriteAdapter"}
// Dependencies: {}
# [doc = " An adapter struct allowing to use `ufmt` on types which implement `core::fmt::Write`"] # [doc = ""] # [doc = " For example:"] # [doc = ""] # [doc = " ```"] # [doc = " use ufmt::uwrite;"] # [doc = " use ufmt_write::uWrite;"] # [doc = " use ufmt_utils::WriteAdapter;"] # [doc = ""] # [doc = " let fancy_number: u8 = 42;"] # [doc = ""] # [doc = " let mut s = String::new();"] # [doc = " uwrite!(WriteAdapter(&mut s), \"{:?}\", fancy_number);"] # [doc = " ```"] pub struct WriteAdapter < W > (pub W) where W : fmt :: Write ;
};
}
