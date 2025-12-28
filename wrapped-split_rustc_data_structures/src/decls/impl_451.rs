macro_rules! deps {
    () => {
        SsoHashMap!();
    };
}

macro_rules! impl_451 {
    () => {
        deps!();
        impl < 'a , K , V > IntoIterator for & 'a mut SsoHashMap < K , V > { type IntoIter = Either < std :: iter :: Map < < & 'a mut ArrayVec < (K , V) , SSO_ARRAY_SIZE > as IntoIterator > :: IntoIter , fn (& 'a mut (K , V)) -> (& 'a K , & 'a mut V) , > , < & 'a mut FxHashMap < K , V > as IntoIterator > :: IntoIter , > ; type Item = < Self :: IntoIter as Iterator > :: Item ; fn into_iter (self) -> Self :: IntoIter { match self { SsoHashMap :: Array (array) => Either :: Left (array . into_iter () . map (adapt_array_mut_it)) , SsoHashMap :: Map (map) => Either :: Right (map . iter_mut ()) , } } }
    };
}

impl_451!();