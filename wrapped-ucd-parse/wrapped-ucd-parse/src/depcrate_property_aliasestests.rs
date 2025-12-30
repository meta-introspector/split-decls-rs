// Generated macro for tests (module)
macro_rules! Depcrate_property_aliasestests {
() => {
// Module: crate::property_aliases
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: PropertyAlias ; # [test] fn parse1 () { let line = "cjkAccountingNumeric     ; kAccountingNumeric\n" ; let row : PropertyAlias = line . parse () . unwrap () ; assert_eq ! (row . abbreviation , "cjkAccountingNumeric") ; assert_eq ! (row . long , "kAccountingNumeric") ; assert ! (row . aliases . is_empty ()) ; } # [test] fn parse2 () { let line = "nv                       ; Numeric_Value\n" ; let row : PropertyAlias = line . parse () . unwrap () ; assert_eq ! (row . abbreviation , "nv") ; assert_eq ! (row . long , "Numeric_Value") ; assert ! (row . aliases . is_empty ()) ; } # [test] fn parse3 () { let line = "scf                      ; Simple_Case_Folding         ; sfc\n" ; let row : PropertyAlias = line . parse () . unwrap () ; assert_eq ! (row . abbreviation , "scf") ; assert_eq ! (row . long , "Simple_Case_Folding") ; assert_eq ! (row . aliases , vec ! ["sfc"]) ; } # [test] fn parse4 () { let line = "cjkRSUnicode             ; kRSUnicode                  ; Unicode_Radical_Stroke; URS\n" ; let row : PropertyAlias = line . parse () . unwrap () ; assert_eq ! (row . abbreviation , "cjkRSUnicode") ; assert_eq ! (row . long , "kRSUnicode") ; assert_eq ! (row . aliases , vec ! ["Unicode_Radical_Stroke" , "URS"]) ; } # [test] fn parse5 () { let line = "isc                      ; ISO_Comment" ; let row : PropertyAlias = line . parse () . unwrap () ; assert_eq ! (row . abbreviation , "isc") ; assert_eq ! (row . long , "ISO_Comment") ; assert ! (row . aliases . is_empty ()) ; } }
};
}
