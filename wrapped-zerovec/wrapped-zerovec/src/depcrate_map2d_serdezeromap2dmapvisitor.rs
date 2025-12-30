// Generated macro for ZeroMap2dMapVisitor (struct)
macro_rules! Depcrate_map2d_serdeZeroMap2dMapVisitor {
() => {
// Module: crate::map2d::serde
// Provides: {"ZeroMap2dMapVisitor"}
// Dependencies: {}
# [doc = " Modified example from https://serde.rs/deserialize-map.html"] struct ZeroMap2dMapVisitor < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + ? Sized + Ord , K1 : ZeroMapKV < 'a > + ? Sized + Ord , V : ZeroMapKV < 'a > + ? Sized , { # [expect (clippy :: type_complexity)] marker : PhantomData < fn () -> (& 'a K0 :: OwnedType , & 'a K1 :: OwnedType , & 'a V :: OwnedType) > , }
};
}
