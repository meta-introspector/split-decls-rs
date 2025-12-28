macro_rules! deps {
    () => {
        ExpandedTest!();
    };
}

macro_rules! filter {
    () => {
        deps!();
        # [allow (clippy :: needless_collect)] fn filter (tests : & mut Vec < ExpandedTest >) { let filters = env :: args_os () . flat_map (OsString :: into_string) . filter_map (| mut arg | { const PREFIX : & str = "trybuild=" ; if arg . starts_with (PREFIX) && arg != PREFIX { Some (arg . split_off (PREFIX . len ())) } else { None } }) . collect :: < Vec < String > > () ; if filters . is_empty () { return ; } tests . retain (| t | { filters . iter () . any (| f | t . test . path . to_string_lossy () . contains (f)) }) ; }
    };
}

filter!();