macro_rules! deps {
    () => {
        StableCrateId!();
    };
}

macro_rules! StableCrateIdMap {
    () => {
        deps!();
        pub type StableCrateIdMap = indexmap :: IndexMap < StableCrateId , CrateNum , BuildHasherDefault < Unhasher > > ;
    };
}

StableCrateIdMap!()