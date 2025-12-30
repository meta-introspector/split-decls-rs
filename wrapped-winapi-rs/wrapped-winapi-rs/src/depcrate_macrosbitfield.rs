// Generated macro for BITFIELD (macro)
macro_rules! Depcrate_macrosBITFIELD {
() => {
// Module: crate::macros
// Provides: {"BITFIELD"}
// Dependencies: {}
macro_rules ! BITFIELD { ($ base : ident $ field : ident : $ fieldtype : ty [$ ($ thing : ident $ set_thing : ident [$ r : expr] ,) +]) => { impl $ base { $ (# [inline] pub fn $ thing (& self) -> $ fieldtype { let size = $ crate :: core :: mem :: size_of ::<$ fieldtype > () * 8 ; self .$ field << (size - $ r . end) >> (size - $ r . end + $ r . start) } # [inline] pub fn $ set_thing (& mut self , val : $ fieldtype) { let mask = ((1 << ($ r . end - $ r . start)) - 1) << $ r . start ; self .$ field &= ! mask ; self .$ field |= (val << $ r . start) & mask ; }) + } } }
};
}
