macro_rules! deps {
    () => {
        MergedCrateInfo!();
    };
}

macro_rules! DepGraphProcessor {
    () => {
        deps!();
        pub trait DepGraphProcessor : Send + Sync { fn process_dep_graph (& self , dot_content : & str ,) -> Result < HashMap < String , MergedCrateInfo > > ; }
    };
}

DepGraphProcessor!()