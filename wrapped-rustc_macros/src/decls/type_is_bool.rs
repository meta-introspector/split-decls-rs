macro_rules! type_is_bool {
    () => {
        # [doc = " Checks whether the type `ty` is `bool`."] pub (crate) fn type_is_bool (ty : & Type) -> bool { type_matches_path (ty , & ["bool"]) }
    };
}

type_is_bool!();