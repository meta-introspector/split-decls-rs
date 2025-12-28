macro_rules! deps {
    () => {
        Target!();
        InlineAsmRegClass!();
        InlineAsmReg!();
        InlineAsmArch!();
    };
}

macro_rules! def_regs {
    () => {
        deps!();
        macro_rules ! def_regs { ($ arch : ident $ arch_reg : ident $ arch_regclass : ident { $ ($ reg : ident : $ class : ident $ (, $ extra_class : ident) * = [$ reg_name : literal $ (, $ alias : literal) *] $ (% $ filter : ident) ?,) * $ (# error = [$ ($ bad_reg : literal) ,+] => $ error : literal ,) * }) => { # [allow (unreachable_code)] # [derive (Copy , Clone , rustc_macros :: Encodable , rustc_macros :: Decodable , Debug , Eq , PartialEq , PartialOrd , Hash , rustc_macros :: HashStable_Generic)] # [allow (non_camel_case_types)] pub enum $ arch_reg { $ ($ reg ,) * } impl $ arch_reg { pub fn name (self) -> &'static str { match self { $ (Self ::$ reg => $ reg_name ,) * } } pub fn reg_class (self) -> $ arch_regclass { match self { $ (Self ::$ reg => $ arch_regclass ::$ class ,) * } } pub fn parse (name : & str) -> Result < Self , &'static str > { match name { $ ($ ($ alias) |* | $ reg_name => Ok (Self ::$ reg) ,) * $ ($ ($ bad_reg) |* => Err ($ error) ,) * _ => Err ("unknown register") , } } pub fn validate (self , _arch : super :: InlineAsmArch , _reloc_model : crate :: spec :: RelocModel , _target_features : & rustc_data_structures :: fx :: FxIndexSet < Symbol >, _target : & crate :: spec :: Target , _is_clobber : bool ,) -> Result < () , &'static str > { match self { $ (Self ::$ reg => { $ ($ filter (_arch , _reloc_model , _target_features , _target , _is_clobber) ?;) ? Ok (()) }) * } } } pub (super) fn fill_reg_map (_arch : super :: InlineAsmArch , _reloc_model : crate :: spec :: RelocModel , _target_features : & rustc_data_structures :: fx :: FxIndexSet < Symbol >, _target : & crate :: spec :: Target , _map : & mut rustc_data_structures :: fx :: FxHashMap < super :: InlineAsmRegClass , rustc_data_structures :: fx :: FxIndexSet < super :: InlineAsmReg >, >,) { # [allow (unused_imports)] use super :: { InlineAsmReg , InlineAsmRegClass } ; $ (if $ ($ filter (_arch , _reloc_model , _target_features , _target , false) . is_ok () &&) ? true { if let Some (set) = _map . get_mut (& InlineAsmRegClass ::$ arch ($ arch_regclass ::$ class)) { set . insert (InlineAsmReg ::$ arch ($ arch_reg ::$ reg)) ; } $ (if let Some (set) = _map . get_mut (& InlineAsmRegClass ::$ arch ($ arch_regclass ::$ extra_class)) { set . insert (InlineAsmReg ::$ arch ($ arch_reg ::$ reg)) ; }) * }) * } } }
    };
}

def_regs!();