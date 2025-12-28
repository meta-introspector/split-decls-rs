macro_rules! deps {
    () => {
        LocalDefId!();
    };
}

macro_rules! macro_122 {
    () => {
        deps!();
        rustc_data_structures :: define_id_collections ! (LocalDefIdMap , LocalDefIdSet , LocalDefIdMapEntry , LocalDefId) ;
    };
}

macro_122!()