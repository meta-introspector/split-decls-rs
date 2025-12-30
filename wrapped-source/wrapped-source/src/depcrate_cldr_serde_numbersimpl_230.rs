// Generated macro for impl_230 (impl)
macro_rules! Depcrate_cldr_serde_numbersimpl_230 {
() => {
// Module: crate::cldr_serde::numbers
// Provides: {"impl_230"}
// Dependencies: {}
impl < 'de > Visitor < 'de > for DecimalFormatVisitor { type Value = DecimalFormat ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a map from keys of the form 10*-count-(zero|one|few|many|other) to compact decimal patterns") } fn visit_map < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : MapAccess < 'de > , { let mut result = DecimalFormat :: default () ; while let Some (key) = access . next_key :: < String > () ? { let (magnitude , count) = key . split ("-count-") . next_tuple () . ok_or_else (| | { M :: Error :: invalid_value (Unexpected :: Str (& key) , & "key to contain -count-") }) ? ; result . patterns . push (CompactDecimalPattern { magnitude : magnitude . to_string () , count : count . to_string () , pattern : access . next_value () ? , }) } Ok (result) } }
};
}
