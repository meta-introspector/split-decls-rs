// Generated macro for ZeroMapMapVisitor (struct)
macro_rules! Depcrate_map_serdeZeroMapMapVisitor {
() => {
// Module: crate::map::serde
// Provides: {"ZeroMapMapVisitor"}
// Dependencies: {}
# [doc = " Modified example from https://serde.rs/deserialize-map.html"] struct ZeroMapMapVisitor < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized + Ord , V : ZeroMapKV < 'a > + ? Sized , { # [expect (clippy :: type_complexity)] marker : PhantomData < fn () -> (& 'a K :: OwnedType , & 'a V :: OwnedType) > , }
};
}
