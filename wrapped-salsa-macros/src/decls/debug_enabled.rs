macro_rules! debug_enabled {
    () => {
        pub (crate) fn debug_enabled (input_name : impl ToString) -> bool { let Some (env_name) = SALSA_DEBUG_MACRO . get_or_init (| | std :: env :: var ("SALSA_DEBUG_MACRO") . ok ()) else { return false ; } ; let input_name = input_name . to_string () ; env_name == "*" || env_name == & input_name [..] }
    };
}

debug_enabled!()