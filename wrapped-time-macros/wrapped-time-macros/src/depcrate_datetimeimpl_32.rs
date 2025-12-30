// Generated macro for impl_32 (impl)
macro_rules! Depcrate_datetimeimpl_32 {
() => {
// Module: crate::datetime
// Provides: {"impl_32"}
// Dependencies: {}
impl ToTokenStream for DateTime { fn append_to (self , ts : & mut TokenStream) { let maybe_offset = match self . offset { Some (offset) => quote_ ! { . assume_offset (# S (offset)) } , None => quote_ ! { } , } ; quote_append ! { ts :: time :: PrimitiveDateTime :: new (# S (self . date) , # S (self . time) ,) # S (maybe_offset) } } }
};
}
