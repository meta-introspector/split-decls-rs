macro_rules! deps {
    () => {
        Table!();
        Value!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < S : Into < String > , V : Into < Self > > From < BTreeMap < S , V > > for Value { fn from (val : BTreeMap < S , V >) -> Self { let table = val . into_iter () . map (| (s , v) | (s . into () , v . into ())) . collect () ; Self :: Table (table) } }
    };
}

impl_59!();