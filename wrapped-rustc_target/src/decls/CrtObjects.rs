macro_rules! CrtObjects {
    () => {
        pub type CrtObjects = BTreeMap < LinkOutputKind , Vec < Cow < 'static , str > > > ;
    };
}

CrtObjects!()