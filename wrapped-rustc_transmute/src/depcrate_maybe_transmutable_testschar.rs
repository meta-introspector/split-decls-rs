// Generated macro for char (module)
macro_rules! Depcrate_maybe_transmutable_testschar {
() => {
// Module: crate::maybe_transmutable::tests
// Provides: {"char"}
// Dependencies: {}
mod char { use super :: * ; use crate :: layout :: tree :: Endian ; # [test] fn should_permit_valid_transmutation () { for order in [Endian :: Big , Endian :: Little] { use Answer :: * ; let char_layout = layout :: Tree :: < Def , ! , ! > :: char (order) ; let no = No (Reason :: DstIsBitIncompatible) ; for (src , answer) in [(0u32 , Yes) , (0xD7FF , Yes) , (0xD800 , no . clone ()) , (0xDFFF , no . clone ()) , (0xE000 , Yes) , (0x10FFFF , Yes) , (0x110000 , no . clone ()) , (0xFFFF0000 , no . clone ()) , (0xFFFFFFFF , no) ,] { let src_layout = layout :: tree :: Tree :: < Def , ! , ! > :: from_big_endian (order , src . to_be_bytes ()) ; let a = is_transmutable (& src_layout , & char_layout , Assume :: default ()) ; assert_eq ! (a , answer , "endian:{order:?},\nsrc:{src:x}") ; } } } }
};
}
