// Generated macro for alloc_support (module)
macro_rules! Depcrate_valuealloc_support {
() => {
// Module: crate::value
// Provides: {"alloc_support"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod alloc_support { use super :: * ; use crate :: std :: boxed :: Box ; impl_value_forward ! ({ impl < T : Value + ? Sized > Value for Box < T > } => x => { ** x }) ; }
};
}
