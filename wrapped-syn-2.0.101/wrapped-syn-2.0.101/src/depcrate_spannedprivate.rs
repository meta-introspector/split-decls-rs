// Generated macro for private (module)
macro_rules! Depcrate_spannedprivate {
() => {
// Module: crate::spanned
// Provides: {"private"}
// Dependencies: {}
mod private { use crate :: spanned :: ToTokens ; pub trait Sealed { } impl < T : ? Sized + ToTokens > Sealed for T { } # [cfg (any (feature = "full" , feature = "derive"))] impl Sealed for crate :: QSelf { } }
};
}
