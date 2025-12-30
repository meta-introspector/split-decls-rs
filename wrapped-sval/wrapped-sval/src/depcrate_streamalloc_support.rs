// Generated macro for alloc_support (module)
macro_rules! Depcrate_streamalloc_support {
() => {
// Module: crate::stream
// Provides: {"alloc_support"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod alloc_support { use super :: * ; use crate :: std :: boxed :: Box ; impl_stream_forward ! ({ impl <'sval , 'a , S : ? Sized > Stream <'sval > for Box < S > where S : Stream <'sval > } => x => { ** x }) ; }
};
}
