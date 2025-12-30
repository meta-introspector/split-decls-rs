// Generated macro for names_to_codepoint (function)
macro_rules! Depcrate_namesnames_to_codepoint {
() => {
// Module: crate::names
// Provides: {"names_to_codepoint"}
// Dependencies: {}
# [doc = " Build one big map in memory from every possible name of a character to its"] # [doc = " corresponding codepoint. One codepoint may be pointed to by multiple names."] # [doc = ""] # [doc = " The return value maps each name to its corresponding codepoint, along with"] # [doc = " a tag associated with how that mapping was generated."] fn names_to_codepoint (data : & BTreeMap < Codepoint , UnicodeData > , aliases : & Option < BTreeMap < Codepoint , Vec < NameAlias > > > , jamo_short_name_table : & [(u32 , & str)] , ideograph : bool , hangul : bool ,) -> BTreeMap < String , (NameTag , u32) > { let mut map = BTreeMap :: new () ; if let Some (ref alias_map) = * aliases { for (cp , aliases) in alias_map { for name_alias in aliases { let v = (NameTag :: Alias , cp . value ()) ; map . insert (name_alias . alias . clone () , v) ; } } } for (cp , datum) in data { let isnull = datum . name . is_empty () || (datum . name . starts_with ('<') && datum . name . ends_with ('>')) ; if ! isnull { let v = (NameTag :: Explicit , cp . value ()) ; map . insert (datum . name . clone () , v) ; } } if ideograph { for & (start , end) in ucd_util :: RANGE_IDEOGRAPH { for cp in start .. end + 1 { let v = (NameTag :: Ideograph , cp) ; map . insert (ucd_util :: ideograph_name (cp) . unwrap () , v) ; } } } if hangul { for & (start , end) in ucd_util :: RANGE_HANGUL_SYLLABLE { for cp in start .. end + 1 { let v = (NameTag :: Hangul , cp) ; map . insert (ucd_util :: hangul_name (jamo_short_name_table , cp) . unwrap () , v ,) ; } } } map }
};
}
