// Generated macro for tests (module)
macro_rules! Depcrate_name_aliasestests {
() => {
// Module: crate::name_aliases
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { NameAlias , NameAliasLabel } ; # [test] fn parse1 () { let line = "0000;NULL;control\n" ; let row : NameAlias = line . parse () . unwrap () ; assert_eq ! (row . codepoint , 0x0) ; assert_eq ! (row . alias , "NULL") ; assert_eq ! (row . label , NameAliasLabel :: Control) ; } # [test] fn parse2 () { let line = "000B;VERTICAL TABULATION;control\n" ; let row : NameAlias = line . parse () . unwrap () ; assert_eq ! (row . codepoint , 0xB) ; assert_eq ! (row . alias , "VERTICAL TABULATION") ; assert_eq ! (row . label , NameAliasLabel :: Control) ; } # [test] fn parse3 () { let line = "0081;HIGH OCTET PRESET;figment\n" ; let row : NameAlias = line . parse () . unwrap () ; assert_eq ! (row . codepoint , 0x81) ; assert_eq ! (row . alias , "HIGH OCTET PRESET") ; assert_eq ! (row . label , NameAliasLabel :: Figment) ; } # [test] fn parse4 () { let line = "E01EF;VS256;abbreviation\n" ; let row : NameAlias = line . parse () . unwrap () ; assert_eq ! (row . codepoint , 0xE01EF) ; assert_eq ! (row . alias , "VS256") ; assert_eq ! (row . label , NameAliasLabel :: Abbreviation) ; } }
};
}
