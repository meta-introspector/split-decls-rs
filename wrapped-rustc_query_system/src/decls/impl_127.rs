macro_rules! deps {
    () => {
        StableHashingContext!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < 'a > HashStable < StableHashingContext < 'a > > for SourceFile { fn hash_stable (& self , hcx : & mut StableHashingContext < 'a > , hasher : & mut StableHasher) { let SourceFile { name : _ , stable_id , cnum , src : _ , ref src_hash , checksum_hash : _ , external_src : _ , start_pos : _ , source_len : _ , lines : _ , ref multibyte_chars , ref normalized_pos , } = * self ; stable_id . hash_stable (hcx , hasher) ; src_hash . hash_stable (hcx , hasher) ; { assert ! (self . lines . read () . is_lines ()) ; let lines = self . lines () ; lines . len () . hash_stable (hcx , hasher) ; for & line in lines . iter () { line . hash_stable (hcx , hasher) ; } } multibyte_chars . len () . hash_stable (hcx , hasher) ; for & char_pos in multibyte_chars . iter () { char_pos . hash_stable (hcx , hasher) ; } normalized_pos . len () . hash_stable (hcx , hasher) ; for & char_pos in normalized_pos . iter () { char_pos . hash_stable (hcx , hasher) ; } cnum . hash_stable (hcx , hasher) ; } }
    };
}

impl_127!()