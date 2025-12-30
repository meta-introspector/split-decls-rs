// Generated macro for style_edition_default (macro)
macro_rules! Depcrate_config_style_editionstyle_edition_default {
() => {
// Module: crate::config::style_edition
// Provides: {"style_edition_default"}
// Dependencies: {}
# [doc = " macro to help implement `StyleEditionDefault` for config options"] # [macro_export] macro_rules ! style_edition_default { ($ ty : ident , $ config_ty : ty , _ => $ default : expr) => { impl $ crate :: config :: style_edition :: StyleEditionDefault for $ ty { type ConfigType = $ config_ty ; fn style_edition_default (_ : $ crate :: config :: StyleEdition) -> Self :: ConfigType { $ default } } } ; ($ ty : ident , $ config_ty : ty , Edition2024 => $ default_2024 : expr , _ => $ default_2015 : expr) => { impl $ crate :: config :: style_edition :: StyleEditionDefault for $ ty { type ConfigType = $ config_ty ; fn style_edition_default (style_edition : $ crate :: config :: StyleEdition ,) -> Self :: ConfigType { match style_edition { $ crate :: config :: StyleEdition :: Edition2015 | $ crate :: config :: StyleEdition :: Edition2018 | $ crate :: config :: StyleEdition :: Edition2021 => $ default_2015 , $ crate :: config :: StyleEdition :: Edition2024 | $ crate :: config :: StyleEdition :: Edition2027 => $ default_2024 , } } } } ; }
};
}
