macro_rules! deps {
    () => {
        Target!();
    };
}

macro_rules! target_reserves_x18 {
    () => {
        deps!();
        pub (crate) fn target_reserves_x18 (target : & Target , target_features : & FxIndexSet < Symbol >) -> bool { target . os == "android" || target . os == "fuchsia" || target . env == "ohos" || target . is_like_darwin || target . is_like_windows || target_features . contains (& sym :: reserve_x18) }
    };
}

target_reserves_x18!();