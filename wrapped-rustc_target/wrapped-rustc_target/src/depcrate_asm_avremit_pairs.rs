// Generated macro for emit_pairs (macro)
macro_rules! Depcrate_asm_avremit_pairs {
() => {
// Module: crate::asm::avr
// Provides: {"emit_pairs"}
// Dependencies: {}
macro_rules ! emit_pairs { ($ self : ident $ modifier : ident , $ ($ pair : ident $ name : literal $ hi : literal $ lo : literal ,) *) => { match ($ self , $ modifier) { $ ((AvrInlineAsmReg ::$ pair , Some ('h')) => $ hi , (AvrInlineAsmReg ::$ pair , Some ('l')) => $ lo , (AvrInlineAsmReg ::$ pair , _) => $ name ,) * _ => $ self . name () , } } ; }
};
}
