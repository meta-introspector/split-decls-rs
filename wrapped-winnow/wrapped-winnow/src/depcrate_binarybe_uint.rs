// Generated macro for be_uint (function)
macro_rules! Depcrate_binarybe_uint {
() => {
// Module: crate::binary
// Provides: {"be_uint"}
// Dependencies: {}
# [inline] fn be_uint < Input , Uint , Error > (input : & mut Input , bound : usize) -> Result < Uint , Error > where Input : StreamIsPartial + Stream < Token = u8 > , Uint : Default + Shl < u8 , Output = Uint > + Add < Uint , Output = Uint > + From < u8 > , Error : ParserError < Input > , { debug_assert_ne ! (bound , 1 , "to_be_uint needs extra work to avoid overflow") ; match input . offset_at (bound) { Ok (offset) => { let res = to_be_uint (input , offset) ; input . next_slice (offset) ; Ok (res) } Err (e) if < Input as StreamIsPartial > :: is_partial_supported () && input . is_partial () => { Err (ParserError :: incomplete (input , e)) } Err (_needed) => Err (ParserError :: from_input (input)) , } }
};
}
