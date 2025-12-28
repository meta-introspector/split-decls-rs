macro_rules! to_profiler_name {
    () => {
        # [doc = " Converts a MIR pass name into a snake case form to match the profiling naming style."] fn to_profiler_name (type_name : & 'static str) -> & 'static str { PASS_TO_PROFILER_NAMES . with (| names | match names . borrow_mut () . entry (type_name) { Entry :: Occupied (e) => * e . get () , Entry :: Vacant (e) => { let snake_case : String = type_name . chars () . flat_map (| c | { if c . is_ascii_uppercase () { vec ! ['_' , c . to_ascii_lowercase ()] } else if c == '-' { vec ! ['_'] } else { vec ! [c] } }) . collect () ; let result = & * String :: leak (format ! ("mir_pass{}" , snake_case)) ; e . insert (result) ; result } }) }
    };
}

to_profiler_name!()