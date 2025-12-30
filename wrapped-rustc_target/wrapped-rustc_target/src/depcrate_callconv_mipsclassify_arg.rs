// Generated macro for classify_arg (function)
macro_rules! Depcrate_callconv_mipsclassify_arg {
() => {
// Module: crate::callconv::mips
// Provides: {"classify_arg"}
// Dependencies: {}
fn classify_arg < Ty , C > (cx : & C , arg : & mut ArgAbi < '_ , Ty > , offset : & mut Size) where C : HasDataLayout , { if ! arg . layout . is_sized () { return ; } let dl = cx . data_layout () ; let size = arg . layout . size ; let align = arg . layout . align . max (dl . i32_align) . min (dl . i64_align) . abi ; if arg . layout . is_aggregate () { let pad_i32 = ! offset . is_aligned (align) ; arg . cast_to_and_pad_i32 (Uniform :: new (Reg :: i32 () , size) , pad_i32) ; } else { arg . extend_integer_width_to (32) ; } * offset = offset . align_to (align) + size . align_to (align) ; }
};
}
