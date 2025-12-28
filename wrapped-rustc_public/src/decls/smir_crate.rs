macro_rules! deps {
    () => {
        CrateNum!();
        BridgeTys!();
        Crate!();
    };
}

macro_rules! smir_crate {
    () => {
        deps!();
        fn smir_crate < 'tcx > (cx : & CompilerCtxt < 'tcx , BridgeTys > , crate_num : rustc_span :: def_id :: CrateNum ,) -> Crate { let name = cx . crate_name (crate_num) ; let is_local = cx . crate_is_local (crate_num) ; let id = cx . crate_num_id (crate_num) ; debug ! (? name , ? crate_num , "smir_crate") ; Crate { id , name , is_local } }
    };
}

smir_crate!();