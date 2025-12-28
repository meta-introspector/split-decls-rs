macro_rules! deps {
    () => {
        SsoHashMap!();
    };
}

macro_rules! impl_450 {
    () => {
        deps!();
        impl < 'a , K , V > IntoIterator for & 'a SsoHashMap < K , V > { type IntoIter = Either < std :: iter :: Map < < & 'a ArrayVec < (K , V) , SSO_ARRAY_SIZE > as IntoIterator > :: IntoIter , fn (& 'a (K , V)) -> (& 'a K , & 'a V) , > , < & 'a FxHashMap < K , V > as IntoIterator > :: IntoIter , > ; type Item = < Self :: IntoIter as Iterator > :: Item ; fn into_iter (self) -> Self :: IntoIter { match self { SsoHashMap :: Array (array) => Either :: Left (array . into_iter () . map (adapt_array_ref_it)) , SsoHashMap :: Map (map) => Either :: Right (map . iter ()) , } } }
    };
}

impl_450!();