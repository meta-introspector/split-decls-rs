macro_rules! deps {
    () => {
        Replacements!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < I : Iterator < Item = char > > Replacements < I > { # [doc = " Create a new iterator that replaces [CJK Compatibility Ideograph] codepoints with normal forms using [Standardized Variation Sequences]."] # [doc = ""] # [doc = " Note that this iterator can also be obtained by directly calling [`.cjk_compat_variants()`] on the iterator."] # [doc = ""] # [doc = " [CJK Compatibility Ideograph]: https://www.unicode.org/glossary/#compatibility_ideograph"] # [doc = " [Standardized Variation Sequences]: https://www.unicode.org/glossary/#standardized_variation_sequence"] # [doc = " [`.cjk_compat_variants()`]: crate::UnicodeNormalization::cjk_compat_variants"] # [inline] pub fn new_cjk_compat_variants (iter : I) -> Replacements < I > { Replacements { iter , buffer : None } } }
    };
}

impl_77!();