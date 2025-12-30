// Generated macro for config_option_with_style_edition_default (macro)
macro_rules! Depcrate_config_optionsconfig_option_with_style_edition_default {
() => {
// Module: crate::config::options
// Provides: {"config_option_with_style_edition_default"}
// Dependencies: {}
# [doc = " Defines unit structs to implement `StyleEditionDefault` for."] # [macro_export] macro_rules ! config_option_with_style_edition_default { ($ name : ident , $ config_ty : ty , _ => $ default : expr) => { # [allow (unreachable_pub)] pub struct $ name ; $ crate :: style_edition_default ! ($ name , $ config_ty , _ => $ default) ; } ; ($ name : ident , $ config_ty : ty , Edition2024 => $ default_2024 : expr , _ => $ default_2015 : expr) => { pub struct $ name ; $ crate :: style_edition_default ! ($ name , $ config_ty , Edition2024 => $ default_2024 , _ => $ default_2015) ; } ; ($ ($ name : ident , $ config_ty : ty , $ (Edition2024 => $ default_2024 : expr ,) ? _ => $ default : expr) ;* $ (;) *) => { $ (config_option_with_style_edition_default ! ($ name , $ config_ty , $ (Edition2024 => $ default_2024 ,) ? _ => $ default) ;) * } ; }
};
}
