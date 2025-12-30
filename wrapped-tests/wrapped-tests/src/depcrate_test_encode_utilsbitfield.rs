// Generated macro for BITFIELD (const)
macro_rules! Depcrate_test_encode_utilsBITFIELD {
() => {
// Module: crate::test_encode_utils
// Provides: {"BITFIELD"}
// Dependencies: {}
# [cfg (feature = "gnustep-1-7")] const BITFIELD : Encoding = Encoding :: Struct ("bitfield" , & [Encoding :: BitField (5 , Some (& (0 , i8 :: ENCODING))) , Encoding :: BitField (0 , Some (& (16 , i16 :: ENCODING))) , Encoding :: BitField (2 , Some (& (16 , i8 :: ENCODING))) ,] ,) ;
};
}
