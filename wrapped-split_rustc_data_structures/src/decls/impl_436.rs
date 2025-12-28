macro_rules! deps {
    () => {
        SortedMap!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        impl < K : Debug , V : Debug > Debug for SortedMap < K , V > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_map () . entries (self . data . iter () . map (| (a , b) | (a , b))) . finish () } }
    };
}

impl_436!()