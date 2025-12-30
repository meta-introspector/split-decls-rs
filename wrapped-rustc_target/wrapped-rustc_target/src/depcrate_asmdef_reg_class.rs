// Generated macro for def_reg_class (macro)
macro_rules! Depcrate_asmdef_reg_class {
() => {
// Module: crate::asm
// Provides: {"def_reg_class"}
// Dependencies: {}
macro_rules ! def_reg_class { ($ arch : ident $ arch_regclass : ident { $ ($ class : ident ,) * }) => { # [derive (Copy , Clone , rustc_macros :: Encodable , rustc_macros :: Decodable , Debug , Eq , PartialEq , PartialOrd , Hash , rustc_macros :: HashStable_Generic)] # [allow (non_camel_case_types)] pub enum $ arch_regclass { $ ($ class ,) * } impl $ arch_regclass { pub fn name (self) -> rustc_span :: Symbol { match self { $ (Self ::$ class => rustc_span :: sym ::$ class ,) * } } pub fn parse (name : rustc_span :: Symbol) -> Result < Self , &'static [rustc_span :: Symbol] > { match name { $ (rustc_span :: sym ::$ class => Ok (Self ::$ class) ,) * _ => Err (& [$ (rustc_span :: sym ::$ class) ,*]) , } } } pub (super) fn regclass_map () -> rustc_data_structures :: fx :: FxHashMap < super :: InlineAsmRegClass , rustc_data_structures :: fx :: FxIndexSet < super :: InlineAsmReg >, > { use rustc_data_structures :: fx :: FxHashMap ; use rustc_data_structures :: fx :: FxIndexSet ; use super :: InlineAsmRegClass ; let mut map = FxHashMap :: default () ; $ (map . insert (InlineAsmRegClass ::$ arch ($ arch_regclass ::$ class) , FxIndexSet :: default ()) ;) * map } } }
};
}
