macro_rules! deps {
    () => {
        SsoHashMap!();
    };
}

macro_rules! impl_446 {
    () => {
        deps!();
        impl < 'a , K , V > Extend < (& 'a K , & 'a V) > for SsoHashMap < K , V > where K : Eq + Hash + Copy , V : Copy , { fn extend < T : IntoIterator < Item = (& 'a K , & 'a V) > > (& mut self , iter : T) { self . extend (iter . into_iter () . map (| (k , v) | (* k , * v))) } # [inline] fn extend_one (& mut self , (& k , & v) : (& 'a K , & 'a V)) { self . insert (k , v) ; } # [inline] fn extend_reserve (& mut self , additional : usize) { Extend :: < (K , V) > :: extend_reserve (self , additional) } }
    };
}

impl_446!();