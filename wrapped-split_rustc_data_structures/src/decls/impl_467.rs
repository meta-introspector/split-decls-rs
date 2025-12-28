macro_rules! deps {
    () => {
        SsoHashSet!();
        SsoHashMap!();
    };
}

macro_rules! impl_467 {
    () => {
        deps!();
        impl < 'a , T > IntoIterator for & 'a SsoHashSet < T > { type IntoIter = std :: iter :: Map < < & 'a SsoHashMap < T , () > as IntoIterator > :: IntoIter , fn ((& 'a T , & 'a ())) -> & 'a T , > ; type Item = < Self :: IntoIter as Iterator > :: Item ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . map . iter () . map (entry_to_key) } }
    };
}

impl_467!();