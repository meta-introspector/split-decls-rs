macro_rules! deps {
    () => {
        StaticCow!();
        LinkerFlavorCli!();
    };
}

macro_rules! LinkArgsCli {
    () => {
        deps!();
        pub type LinkArgsCli = BTreeMap < LinkerFlavorCli , Vec < StaticCow < str > > > ;
    };
}

LinkArgsCli!();