// Generated macro for nonzero (module)
macro_rules! Depcrate_maybe_transmutable_testsnonzero {
() => {
// Module: crate::maybe_transmutable::tests
// Provides: {"nonzero"}
// Dependencies: {}
mod nonzero { use super :: * ; use crate :: { Answer , Reason } ; const NONZERO_BYTE_WIDTHS : [u64 ; 5] = [1 , 2 , 4 , 8 , 16] ; # [test] fn should_permit_identity_transmutation () { for width in NONZERO_BYTE_WIDTHS { let layout = layout :: Tree :: < Def , ! , ! > :: nonzero (width) ; assert_eq ! (is_transmutable (& layout , & layout , Assume :: default ()) , Answer :: Yes) ; } } # [test] fn should_permit_valid_transmutation () { for width in NONZERO_BYTE_WIDTHS { use Answer :: * ; let num = layout :: Tree :: < Def , ! , ! > :: number (width) ; let nz = layout :: Tree :: < Def , ! , ! > :: nonzero (width) ; let a = is_transmutable (& num , & nz , Assume :: default ()) ; assert_eq ! (a , No (Reason :: DstIsBitIncompatible) , "width:{width}") ; let a = is_transmutable (& nz , & num , Assume :: default ()) ; assert_eq ! (a , Yes , "width:{width}") ; } } }
};
}
