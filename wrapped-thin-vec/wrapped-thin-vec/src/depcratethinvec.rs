// Generated macro for ThinVec (struct)
macro_rules! DepcrateThinVec {
() => {
// Module: crate
// Provides: {"ThinVec"}
// Dependencies: {}
# [doc = " See the crate's top level documentation for a description of this type."] # [repr (C)] pub struct ThinVec < T > { ptr : NonNull < Header > , boo : PhantomData < T > , }
};
}
