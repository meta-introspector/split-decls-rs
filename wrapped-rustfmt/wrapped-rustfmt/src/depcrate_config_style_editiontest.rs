// Generated macro for test (module)
macro_rules! Depcrate_config_style_editiontest {
() => {
// Module: crate::config::style_edition
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: config :: StyleEdition ; # [test] fn test_impl_default_style_edition_struct_for_all_editions () { struct Unit ; style_edition_default ! (Unit , usize , _ => 100) ; assert_eq ! (Unit :: style_edition_default (StyleEdition :: Edition2015) , 100) ; assert_eq ! (Unit :: style_edition_default (StyleEdition :: Edition2018) , 100) ; assert_eq ! (Unit :: style_edition_default (StyleEdition :: Edition2021) , 100) ; assert_eq ! (Unit :: style_edition_default (StyleEdition :: Edition2024) , 100) ; assert_eq ! (Unit :: style_edition_default (StyleEdition :: Edition2027) , 100) ; } # [test] fn test_impl_default_style_edition_for_old_and_new_editions () { struct Unit ; style_edition_default ! (Unit , usize , Edition2024 => 50 , _ => 100) ; assert_eq ! (Unit :: style_edition_default (StyleEdition :: Edition2015) , 100) ; assert_eq ! (Unit :: style_edition_default (StyleEdition :: Edition2018) , 100) ; assert_eq ! (Unit :: style_edition_default (StyleEdition :: Edition2021) , 100) ; assert_eq ! (Unit :: style_edition_default (StyleEdition :: Edition2024) , 50) ; assert_eq ! (Unit :: style_edition_default (StyleEdition :: Edition2027) , 50) ; } }
};
}
