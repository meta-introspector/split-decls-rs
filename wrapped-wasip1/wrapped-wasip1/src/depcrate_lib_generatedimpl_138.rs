// Generated macro for impl_138 (impl)
macro_rules! Depcrate_lib_generatedimpl_138 {
() => {
// Module: crate::lib_generated
// Provides: {"impl_138"}
// Dependencies: {}
impl Whence { pub const fn raw (& self) -> u8 { self . 0 } pub fn name (& self) -> & 'static str { match self . 0 { 0 => "SET" , 1 => "CUR" , 2 => "END" , _ => unsafe { core :: hint :: unreachable_unchecked () } , } } pub fn message (& self) -> & 'static str { match self . 0 { 0 => "Seek relative to start-of-file." , 1 => "Seek relative to current position." , 2 => "Seek relative to end-of-file." , _ => unsafe { core :: hint :: unreachable_unchecked () } , } } }
};
}
