// Generated macro for impl_326 (impl)
macro_rules! Depcrate_unicode_dataimpl_326 {
() => {
// Module: crate::unicode_data
// Provides: {"impl_326"}
// Dependencies: {}
impl std :: fmt :: Display for UnicodeData { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{};" , self . codepoint) ? ; write ! (f , "{};" , self . name) ? ; write ! (f , "{};" , self . general_category) ? ; write ! (f , "{};" , self . canonical_combining_class) ? ; write ! (f , "{};" , self . bidi_class) ? ; if self . decomposition . is_canonical () && self . decomposition . mapping () == & [self . codepoint] { write ! (f , ";") ? ; } else { write ! (f , "{};" , self . decomposition) ? ; } if let Some (n) = self . numeric_type_decimal { write ! (f , "{};" , n) ? ; } else { write ! (f , ";") ? ; } if let Some (n) = self . numeric_type_digit { write ! (f , "{};" , n) ? ; } else { write ! (f , ";") ? ; } if let Some (n) = self . numeric_type_numeric { write ! (f , "{};" , n) ? ; } else { write ! (f , ";") ? ; } write ! (f , "{};" , if self . bidi_mirrored { "Y" } else { "N" }) ? ; write ! (f , "{};" , self . unicode1_name) ? ; write ! (f , "{};" , self . iso_comment) ? ; if let Some (cp) = self . simple_uppercase_mapping { write ! (f , "{};" , cp) ? ; } else { write ! (f , ";") ? ; } if let Some (cp) = self . simple_lowercase_mapping { write ! (f , "{};" , cp) ? ; } else { write ! (f , ";") ? ; } if let Some (cp) = self . simple_titlecase_mapping { write ! (f , "{}" , cp) ? ; } Ok (()) } }
};
}
