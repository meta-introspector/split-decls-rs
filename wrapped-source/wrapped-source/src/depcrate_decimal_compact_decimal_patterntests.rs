// Generated macro for tests (module)
macro_rules! Depcrate_decimal_compact_decimal_patterntests {
() => {
// Module: crate::decimal::compact_decimal_pattern
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use icu_provider :: prelude :: * ; use zerofrom :: ZeroFrom ; use zerovec :: ule :: AsULE ; # [test] fn test_french_compressibility () { let cldr_42_long_french_data = CompactDecimalPatternData :: try_from (& serde_json :: from_str :: < DecimalFormat > (r#"
                {
                    "1000-count-1": "mille",
                    "1000-count-one": "0 millier",
                    "1000-count-other": "0 mille",
                    "10000-count-one": "00 mille",
                    "10000-count-other": "00 mille",
                    "100000-count-one": "000 mille",
                    "100000-count-other": "000 mille"
                }
            "# ,) . unwrap () ,) . unwrap () ; let cldr_42_long_french : Box < [(i8 , Count , Pattern)] > = cldr_42_long_french_data . patterns . iter0 () . flat_map (| kkv | { let key0 = * kkv . key0 () ; kkv . into_iter1 () . map (move | (k , v) | (key0 , Count :: from_unaligned (* k) , Pattern :: zero_from (v))) }) . collect () ; assert_eq ! (cldr_42_long_french . as_ref () , [(3 , Count :: One , Pattern { index : 0 , exponent : 3 , literal_text : Cow :: Borrowed (" millier") }) , (3 , Count :: Other , Pattern { index : 0 , exponent : 3 , literal_text : Cow :: Borrowed (" mille") }) , (3 , Count :: Explicit1 , Pattern { index : 255 , exponent : 3 , literal_text : Cow :: Borrowed ("mille") }) , (4 , Count :: Other , Pattern { index : 0 , exponent : 3 , literal_text : Cow :: Borrowed (" mille") }) ,]) ; let compressible_long_french_data = CompactDecimalPatternData :: try_from (& serde_json :: from_str :: < DecimalFormat > (r#"
                {
                    "1000-count-1": "mille",
                    "1000-count-one": "0 mille",
                    "1000-count-other": "0 mille",
                    "10000-count-one": "00 mille",
                    "10000-count-other": "00 mille",
                    "100000-count-one": "000 mille",
                    "100000-count-other": "000 mille"
                }
            "# ,) . unwrap () ,) . unwrap () ; let compressible_long_french : Box < [(i8 , Count , Pattern)] > = compressible_long_french_data . patterns . iter0 () . flat_map (| kkv | { let key0 = * kkv . key0 () ; kkv . into_iter1 () . map (move | (k , v) | (key0 , Count :: from_unaligned (* k) , Pattern :: zero_from (v))) }) . collect () ; assert_eq ! (compressible_long_french . as_ref () , [(3 , Count :: Other , Pattern { index : 0 , exponent : 3 , literal_text : Cow :: Borrowed (" mille") }) , (3 , Count :: Explicit1 , Pattern { index : 255 , exponent : 3 , literal_text : Cow :: Borrowed ("mille") }) ,]) ; } # [test] fn test_holes () { let spanish_data = CompactDecimalPatternData :: try_from (& serde_json :: from_str :: < DecimalFormat > (r#"
                {
                    "1000-count-one": "0 mil",
                    "1000-count-other": "0 mil",
                    "10000-count-one": "00 mil",
                    "10000-count-other": "00 mil",
                    "100000-count-one": "000 mil",
                    "100000-count-other": "000 mil",
                    "1000000-count-one": "0 M",
                    "1000000-count-other": "0 M",
                    "10000000-count-one": "00 M",
                    "10000000-count-other": "00 M",
                    "100000000-count-one": "000 M",
                    "100000000-count-other": "000 M",
                    "1000000000-count-one": "0000 M",
                    "1000000000-count-other": "0000 M",
                    "10000000000-count-one": "00 mil M",
                    "10000000000-count-other": "00 mil M",
                    "100000000000-count-one": "000 mil M",
                    "100000000000-count-other": "000 mil M"
                }
            "# ,) . unwrap () ,) . unwrap () ; let spanish : Box < [(i8 , Count , Pattern)] > = spanish_data . patterns . iter0 () . flat_map (| kkv | { let key0 = * kkv . key0 () ; kkv . into_iter1 () . map (move | (k , v) | (key0 , Count :: from_unaligned (* k) , Pattern :: zero_from (v))) }) . collect () ; assert_eq ! (spanish . as_ref () , [(3 , Count :: Other , Pattern { index : 0 , exponent : 3 , literal_text : Cow :: Borrowed (" mil") }) , (6 , Count :: Other , Pattern { index : 0 , exponent : 6 , literal_text : Cow :: Borrowed (" M") }) , (10 , Count :: Other , Pattern { index : 0 , exponent : 9 , literal_text : Cow :: Borrowed (" mil M") }) ,]) ; } # [test] fn test_pattern_syntax_errors () { assert_eq ! (parse ("M.") . err () . unwrap () , "Unsupported symbol in compact decimal pattern M.") ; assert_eq ! (parse ("M'.'") . unwrap () . unwrap () . literal_text , "M.") ; assert_eq ! (parse ("0 0") . err () . unwrap () , "Multiple placeholders in compact decimal pattern 0 0") ; assert_eq ! (parse ("0 '0'") . unwrap () . unwrap () . literal_text , " 0") ; let zeros = str :: repeat ("0" , 256) ; assert_eq ! (parse (& zeros [.. 128]) . err () . unwrap () , String :: from ("Too many 0s in pattern ") + & zeros [.. 128]) ; assert_eq ! (parse (& zeros [.. 127]) . unwrap () . unwrap () . literal_text , "") ; } # [test] fn test_inter_pattern_errors () { assert_eq ! (CompactDecimalPatternData :: try_from (& serde_json :: from_str ::< DecimalFormat > (r#"{ "1000-count-other": "0k", "1000-count-other": "0K" }"# ,) . unwrap () ,) . err () . unwrap () , "Plural case Other is duplicated for type 10^3") ; assert_eq ! (CompactDecimalPatternData :: try_from (& serde_json :: from_str ::< DecimalFormat > (r#"{ "1-count-one": "0" }"#) . unwrap ()) . err () . unwrap () , "Missing other case for type 10^0") ; assert_eq ! (CompactDecimalPatternData :: try_from (& serde_json :: from_str ::< DecimalFormat > (r#"{ "1000-count-one": "0k", "1000-count-other": "0" }"#) . unwrap ()) . err () . unwrap () , "Non-0 pattern for type 10^3 whose pattern for count=other is 0") ; assert_eq ! (CompactDecimalPatternData :: try_from (& serde_json :: from_str ::< DecimalFormat > (r#"{ "1000-count-other": "k" }"#) . unwrap ()) . err () . unwrap () , "Missing placeholder in other case of type 10^3") ; assert_eq ! (CompactDecimalPatternData :: try_from (& serde_json :: from_str ::< DecimalFormat > (r#"
                        {
                            "10000-count-other": "00 thousand",
                            "10000-count-one": "0 myriad"
                        }
                    "# ,) . unwrap () ,) . err () . unwrap () , "Inconsistent placeholders within type 10^4: 2 0s for other, 1 0s for One") ; assert_eq ! (CompactDecimalPatternData :: try_from (& serde_json :: from_str ::< DecimalFormat > (r#"{ "1000-count-other": "00000 tenths" }"#) . unwrap ()) . err () . unwrap () , "Too many 0s in type 10^3, (5, implying nonpositive exponent c=-1)") ; let long_pattern = format ! ("thous{}nds (0)" , str :: repeat ("a" , 244)) ; let overlong_pattern = format ! ("thous{}nds (0)" , str :: repeat ("a" , 245)) ; assert_eq ! (CompactDecimalPatternData :: try_from (& serde_json :: from_str ::< DecimalFormat > (format ! (r#"{{ "1000-count-other": "{overlong_pattern}" }}"#) . as_str ()) . unwrap ()) . err () . unwrap () , "Placeholder index is too large in type=10^3, count=Other") ; assert_eq ! (CompactDecimalPatternData :: try_from (& serde_json :: from_str ::< DecimalFormat > (format ! (r#"{{ "1000-count-other": "{long_pattern}" }}"#) . as_str ()) . unwrap ()) . unwrap () . patterns . get0 (& 3) . and_then (| plural_map | plural_map . get1 (& Count :: Other)) . unwrap () . index , 254) ; } }
};
}
