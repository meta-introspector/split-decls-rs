// Generated macro for impl_162 (impl)
macro_rules! Depcrate_lib_generatedimpl_162 {
() => {
// Module: crate::lib_generated
// Provides: {"impl_162"}
// Dependencies: {}
impl Advice { pub const fn raw (& self) -> u8 { self . 0 } pub fn name (& self) -> & 'static str { match self . 0 { 0 => "NORMAL" , 1 => "SEQUENTIAL" , 2 => "RANDOM" , 3 => "WILLNEED" , 4 => "DONTNEED" , 5 => "NOREUSE" , _ => unsafe { core :: hint :: unreachable_unchecked () } , } } pub fn message (& self) -> & 'static str { match self . 0 { 0 => "The application has no advice to give on its behavior with respect to the specified data." , 1 => "The application expects to access the specified data sequentially from lower offsets to higher offsets." , 2 => "The application expects to access the specified data in a random order." , 3 => "The application expects to access the specified data in the near future." , 4 => "The application expects that it will not access the specified data in the near future." , 5 => "The application expects to access the specified data once and then not reuse it thereafter." , _ => unsafe { core :: hint :: unreachable_unchecked () } , } } }
};
}
