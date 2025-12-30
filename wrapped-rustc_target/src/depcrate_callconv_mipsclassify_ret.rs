// Generated macro for classify_ret (function)
macro_rules! Depcrate_callconv_mipsclassify_ret {
() => {
// Module: crate::callconv::mips
// Provides: {"classify_ret"}
// Dependencies: {}
fn classify_ret < Ty , C > (cx : & C , ret : & mut ArgAbi < '_ , Ty > , offset : & mut Size) where C : HasDataLayout , { if ! ret . layout . is_aggregate () { ret . extend_integer_width_to (32) ; } else { ret . make_indirect () ; * offset += cx . data_layout () . pointer_size () ; } }
};
}
