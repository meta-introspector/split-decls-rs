macro_rules! deps {
    () => {
        Table!();
        Value!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < S : Into < String > + Hash + Eq , V : Into < Self > > From < HashMap < S , V > > for Value { fn from (val : HashMap < S , V >) -> Self { let table = val . into_iter () . map (| (s , v) | (s . into () , v . into ())) . collect () ; Self :: Table (table) } }
    };
}

impl_60!();