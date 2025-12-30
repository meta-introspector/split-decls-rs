// Generated macro for impl_236 (impl)
macro_rules! Depcrate_cldr_serde_numbersimpl_236 {
() => {
// Module: crate::cldr_serde::numbers
// Provides: {"impl_236"}
// Dependencies: {}
impl < 'de > Visitor < 'de > for NumberingSystemDataVisitor { type Value = NumberingSystemData ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("formatting data by numbering system") } fn visit_map < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : MapAccess < 'de > , { let mut result = NumberingSystemData :: default () ; while let Some (key) = access . next_key :: < String > () ? { let (stype , _ , numsys) = match key . split ('-') . next_tuple () { Some (v) => v , None => continue , } ; match stype { "symbols" => { let value : Symbols = access . next_value () ? ; result . symbols . insert (numsys . to_string () , value) ; } "decimalFormats" => { let value : DecimalFormats = access . next_value () ? ; result . formats . insert (numsys . to_string () , value) ; } "currencyFormats" => { let value : CurrencyFormattingPatterns = access . next_value () ? ; result . currency_patterns . insert (numsys . to_string () , value) ; } "percentFormats" => { let value : PercentFormattingPatterns = access . next_value () ? ; result . percent_patterns . insert (numsys . to_string () , value) ; } _ => { } } } Ok (result) } }
};
}
