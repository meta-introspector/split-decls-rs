macro_rules! deps {
    () => {
        EncodedPoint!();
        ModulusSize!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        # [cfg (feature = "subtle")] impl < Size > ConditionallySelectable for EncodedPoint < Size > where Size : ModulusSize , < Size :: UncompressedPointSize as ArraySize > :: ArrayType < u8 > : Copy , { fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { let mut bytes = Array :: default () ; for (i , byte) in bytes . iter_mut () . enumerate () { * byte = u8 :: conditional_select (& a . bytes [i] , & b . bytes [i] , choice) ; } Self { bytes } } }
    };
}

impl_7!();