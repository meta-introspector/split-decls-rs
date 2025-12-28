macro_rules! deps {
    () => {
        StaticCow!();
        LinkerFlavor!();
    };
}

macro_rules! LinkArgs {
    () => {
        deps!();
        pub type LinkArgs = BTreeMap < LinkerFlavor , Vec < StaticCow < str > > > ;
    };
}

LinkArgs!()