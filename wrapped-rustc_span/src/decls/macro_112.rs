macro_rules! deps {
    () => {
        DefId!();
    };
}

macro_rules! macro_112 {
    () => {
        deps!();
        rustc_data_structures :: define_id_collections ! (DefIdMap , DefIdSet , DefIdMapEntry , DefId) ;
    };
}

macro_112!();