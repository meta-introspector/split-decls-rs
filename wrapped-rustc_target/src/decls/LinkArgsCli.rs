macro_rules! deps {
    () => {
        LinkerFlavorCli!();
        StaticCow!();
    };
}

macro_rules! LinkArgsCli {
    () => {
        deps!();
        pub type LinkArgsCli = BTreeMap < LinkerFlavorCli , Vec < StaticCow < str > > > ;
    };
}

LinkArgsCli!()