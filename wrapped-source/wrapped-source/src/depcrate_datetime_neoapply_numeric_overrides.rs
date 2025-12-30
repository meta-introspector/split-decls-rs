// Generated macro for apply_numeric_overrides (function)
macro_rules! Depcrate_datetime_neoapply_numeric_overrides {
() => {
// Module: crate::datetime::neo
// Provides: {"apply_numeric_overrides"}
// Dependencies: {}
# [doc = " Given a lengthpattern, apply any numeric overrides it may have to `pattern`"] # [allow (dead_code)] fn apply_numeric_overrides (lp : & ca :: LengthPattern , pattern : & mut pattern :: runtime :: Pattern) { use icu :: datetime :: provider :: fields :: { self , FieldLength , FieldNumericOverrides :: * , FieldSymbol , } ; let ca :: LengthPattern :: WithNumberingSystems { ref numbering_systems , .. } = * lp else { return ; } ; let (numeric , symbol_to_replace) = match & * * numbering_systems { "hanidec" => (Hanidec , None) , "hebr" => (Hebr , None) , "d=hanidays" => (Hanidays , Some (FieldSymbol :: Day (fields :: Day :: DayOfMonth))) , "M=romanlow" => (Romanlow , Some (FieldSymbol :: Month (fields :: Month :: Format))) , "y=jpanyear" => (Jpnyear , Some (FieldSymbol :: Year (fields :: Year :: Calendar))) , _ => panic ! ("Found unexpected numeric override {numbering_systems}") , } ; pattern . items . for_each_mut (| item | { if let pattern :: PatternItem :: Field (ref mut field) = * item { if field . length != FieldLength :: One { assert ! (field . length != FieldLength :: Two || symbol_to_replace != Some (field . symbol) , "We don't know what to do when there is a non-targeted numeric override \
                         on a two-digit numeric field") ; return ; } if let Some (symbol) = symbol_to_replace { if symbol != field . symbol { return ; } } field . length = FieldLength :: NumericOverride (numeric) ; } }) }
};
}
