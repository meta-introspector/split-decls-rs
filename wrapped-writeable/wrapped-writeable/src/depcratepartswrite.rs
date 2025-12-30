// Generated macro for PartsWrite (trait)
macro_rules! DepcratePartsWrite {
() => {
// Module: crate
// Provides: {"PartsWrite"}
// Dependencies: {}
# [doc = " A sink that supports annotating parts of the string with [`Part`]s."] pub trait PartsWrite : fmt :: Write { type SubPartsWrite : PartsWrite + ? Sized ; # [doc = " Annotates all strings written by the closure with the given [`Part`]."] fn with_part (& mut self , part : Part , f : impl FnMut (& mut Self :: SubPartsWrite) -> fmt :: Result ,) -> fmt :: Result ; }
};
}
