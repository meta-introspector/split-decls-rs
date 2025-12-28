macro_rules! deps {
    () => {
        LinkerFlavor!();
        StaticCow!();
    };
}

macro_rules! LinkArgs {
    () => {
        deps!();
        pub type LinkArgs = BTreeMap < LinkerFlavor , Vec < StaticCow < str > > > ;
    };
}

LinkArgs!();