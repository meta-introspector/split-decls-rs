// Generated macro for impl_101 (impl)
macro_rules! Depcrate_data_runtimeimpl_101 {
() => {
// Module: crate::data::runtime
// Provides: {"impl_101"}
// Dependencies: {}
impl StrLitKind { fn write_start (self , w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { match self { Self :: Normal => write ! (w , "\"") , Self :: Raw (n) => { write ! (w , "r") ? ; for _ in 0 .. n { write ! (w , "#") ? ; } write ! (w , "\"") } } } fn write_end (self , w : & mut impl std :: fmt :: Write) -> std :: fmt :: Result { match self { Self :: Normal => write ! (w , "\"") , Self :: Raw (n) => { write ! (w , "\"") ? ; for _ in 0 .. n { write ! (w , "#") ? ; } Ok (()) } } } }
};
}
