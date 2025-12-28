macro_rules! deps {
    () => {
        Disambiguator!();
        DisambiguatorMap!();
        IdentityHash!();
    };
}

macro_rules! impl_366 {
    () => {
        deps!();
        impl DisambiguatorMap { pub (crate) fn disambiguate (& mut self , key : IdentityHash) -> Disambiguator { use hashbrown :: hash_map :: RawEntryMut ; let entry = self . map . raw_entry_mut () . from_hash (key . hash , | k | * k == key) ; let disambiguator = match entry { RawEntryMut :: Occupied (occupied) => occupied . into_mut () , RawEntryMut :: Vacant (vacant) => { vacant . insert_with_hasher (key . hash , key , Disambiguator (0) , | k | k . hash) . 1 } } ; let result = * disambiguator ; disambiguator . 0 += 1 ; result } pub fn clear (& mut self) { self . map . clear () } pub fn is_empty (& self) -> bool { self . map . is_empty () } }
    };
}

impl_366!();