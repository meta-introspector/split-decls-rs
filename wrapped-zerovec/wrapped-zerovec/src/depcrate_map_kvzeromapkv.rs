// Generated macro for ZeroMapKV (trait)
macro_rules! Depcrate_map_kvZeroMapKV {
() => {
// Module: crate::map::kv
// Provides: {"ZeroMapKV"}
// Dependencies: {}
# [doc = " Trait marking types which are allowed to be keys or values in [`ZeroMap`](super::ZeroMap)."] # [doc = ""] # [doc = " Users should not be calling methods of this trait directly, however if you are"] # [doc = " implementing your own [`AsULE`] or [`VarULE`] type you may wish to implement"] # [doc = " this trait."] pub trait ZeroMapKV < 'a > { # [doc = " The container that can be used with this type: [`ZeroVec`] or [`VarZeroVec`]."] type Container : MutableZeroVecLike < 'a , Self , SliceVariant = Self :: Slice , GetType = Self :: GetType , OwnedType = Self :: OwnedType , > + Sized ; type Slice : ZeroVecLike < Self , GetType = Self :: GetType > + ? Sized ; # [doc = " The type produced by `Container::get()`"] # [doc = ""] # [doc = " This type will be predetermined by the choice of `Self::Container`:"] # [doc = " For sized types this must be `T::ULE`, and for unsized types this must be `T`"] type GetType : ? Sized + 'static ; # [doc = " The type produced by `Container::replace()` and `Container::remove()`,"] # [doc = " also used during deserialization. If `Self` is human readable serialized,"] # [doc = " deserializing to `Self::OwnedType` should produce the same value once"] # [doc = " passed through `Self::owned_as_self()`"] # [doc = ""] # [doc = " This type will be predetermined by the choice of `Self::Container`:"] # [doc = " For sized types this must be `T` and for unsized types this must be `Box<T>`"] type OwnedType : 'static ; }
};
}
