// Generated macro for adapters (module)
macro_rules! Depcrateadapters {
() => {
// Module: crate
// Provides: {"adapters"}
// Dependencies: {}
# [doc = " Helper types for trait impls."] pub mod adapters { use super :: * ; pub use parts_write_adapter :: CoreWriteAsPartsWrite ; pub use parts_write_adapter :: WithPart ; pub use try_writeable :: TryWriteableInfallibleAsWriteable ; pub use try_writeable :: WriteableAsTryWriteableInfallible ; # [derive (Debug)] # [allow (clippy :: exhaustive_structs)] pub struct LossyWrap < T > (pub T) ; impl < T : TryWriteable > Writeable for LossyWrap < T > { fn write_to < W : fmt :: Write + ? Sized > (& self , sink : & mut W) -> fmt :: Result { let _ = self . 0 . try_write_to (sink) ? ; Ok (()) } fn writeable_length_hint (& self) -> LengthHint { self . 0 . writeable_length_hint () } } impl < T : TryWriteable > fmt :: Display for LossyWrap < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let _ = self . 0 . try_write_to (f) ? ; Ok (()) } } }
};
}
