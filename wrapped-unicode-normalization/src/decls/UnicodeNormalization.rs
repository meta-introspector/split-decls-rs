macro_rules! deps {
    () => {
        Recompositions!();
        Replacements!();
        StreamSafe!();
        Decompositions!();
    };
}

macro_rules! UnicodeNormalization {
    () => {
        deps!();
        # [doc = " Methods for iterating over strings while applying Unicode normalizations"] # [doc = " as described in"] # [doc = " [Unicode Standard Annex #15](http://www.unicode.org/reports/tr15/)."] pub trait UnicodeNormalization < I : Iterator < Item = char > > { # [doc = " Returns an iterator over the string in Unicode Normalization Form D"] # [doc = " (canonical decomposition)."] fn nfd (self) -> Decompositions < I > ; # [doc = " Returns an iterator over the string in Unicode Normalization Form KD"] # [doc = " (compatibility decomposition)."] fn nfkd (self) -> Decompositions < I > ; # [doc = " An Iterator over the string in Unicode Normalization Form C"] # [doc = " (canonical decomposition followed by canonical composition)."] fn nfc (self) -> Recompositions < I > ; # [doc = " An Iterator over the string in Unicode Normalization Form KC"] # [doc = " (compatibility decomposition followed by canonical composition)."] fn nfkc (self) -> Recompositions < I > ; # [doc = " A transformation which replaces [CJK Compatibility Ideograph] codepoints"] # [doc = " with normal forms using [Standardized Variation Sequences]. This is not"] # [doc = " part of the canonical or compatibility decomposition algorithms, but"] # [doc = " performing it before those algorithms produces normalized output which"] # [doc = " better preserves the intent of the original text."] # [doc = ""] # [doc = " Note that many systems today ignore variation selectors, so these"] # [doc = " may not immediately help text display as intended, but they at"] # [doc = " least preserve the information in a standardized form, giving"] # [doc = " implementations the option to recognize them."] # [doc = ""] # [doc = " [CJK Compatibility Ideograph]: https://www.unicode.org/glossary/#compatibility_ideograph"] # [doc = " [Standardized Variation Sequences]: https://www.unicode.org/glossary/#standardized_variation_sequence"] fn cjk_compat_variants (self) -> Replacements < I > ; # [doc = " An Iterator over the string with Conjoining Grapheme Joiner characters"] # [doc = " inserted according to the Stream-Safe Text Process ([UAX15-D4])."] # [doc = ""] # [doc = " [UAX15-D4]: https://www.unicode.org/reports/tr15/#UAX15-D4"] fn stream_safe (self) -> StreamSafe < I > ; }
    };
}

UnicodeNormalization!();