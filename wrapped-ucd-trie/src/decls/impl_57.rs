macro_rules! deps {
    () => {
        TrieSetSlice!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < 'a > TrieSetSlice < 'a > { # [doc = " Returns true if and only if the given Unicode scalar value is in this"] # [doc = " set."] pub fn contains_char (& self , c : char) -> bool { self . contains (c as usize) } # [doc = " Returns true if and only if the given codepoint is in this set."] # [doc = ""] # [doc = " If the given value exceeds the codepoint range (i.e., it's greater"] # [doc = " than `0x10FFFF`), then this returns false."] pub fn contains_u32 (& self , cp : u32) -> bool { if cp > 0x10FFFF { return false ; } self . contains (cp as usize) } # [inline (always)] fn contains (& self , cp : usize) -> bool { if cp < 0x800 { self . chunk_contains (cp , self . tree1_level1 [cp >> 6]) } else if cp < 0x10000 { let leaf = match self . tree2_level1 . get ((cp >> 6) - 0x20) { None => return false , Some (& leaf) => leaf , } ; self . chunk_contains (cp , self . tree2_level2 [leaf as usize]) } else { let child = match self . tree3_level1 . get ((cp >> 12) - 0x10) { None => return false , Some (& child) => child , } ; let i = ((child as usize) * CHUNK_SIZE) + ((cp >> 6) & 0b111111) ; let leaf = self . tree3_level2 [i] ; self . chunk_contains (cp , self . tree3_level3 [leaf as usize]) } } # [inline (always)] fn chunk_contains (& self , cp : usize , chunk : u64) -> bool { ((chunk >> (cp & 0b111111)) & 1) == 1 } }
    };
}

impl_57!();