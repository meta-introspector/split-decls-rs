macro_rules! deps {
    () => {
        UnicodeNormalization!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < I : Iterator < Item = char > > UnicodeNormalization < I > for I { # [inline] fn nfd (self) -> Decompositions < I > { Decompositions :: new_canonical (self) } # [inline] fn nfkd (self) -> Decompositions < I > { Decompositions :: new_compatible (self) } # [inline] fn nfc (self) -> Recompositions < I > { Recompositions :: new_canonical (self) } # [inline] fn nfkc (self) -> Recompositions < I > { Recompositions :: new_compatible (self) } # [inline] fn cjk_compat_variants (self) -> Replacements < I > { Replacements :: new_cjk_compat_variants (self) } # [inline] fn stream_safe (self) -> StreamSafe < I > { StreamSafe :: new (self) } }
    };
}

impl_18!()