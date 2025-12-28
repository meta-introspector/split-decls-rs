macro_rules! deps {
    () => {
        Target!();
    };
}

macro_rules! frame_pointer_is_r7 {
    () => {
        deps!();
        fn frame_pointer_is_r7 (target_features : & FxIndexSet < Symbol > , target : & Target) -> bool { target . is_like_darwin || (! target . is_like_windows && target_features . contains (& sym :: thumb_mode)) }
    };
}

frame_pointer_is_r7!()