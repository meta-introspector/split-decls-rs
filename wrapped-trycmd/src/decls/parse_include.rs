macro_rules! parse_include {
    () => {
        # [allow (clippy :: needless_collect)] fn parse_include (args : impl IntoIterator < Item = std :: ffi :: OsString >) -> Option < Vec < String > > { let filters = args . into_iter () . flat_map (std :: ffi :: OsString :: into_string) . filter_map (| arg | { const PREFIX : & str = "trycmd=" ; if let Some (remainder) = arg . strip_prefix (PREFIX) { if remainder . is_empty () { None } else { Some (remainder . to_owned ()) } } else { None } }) . collect :: < Vec < String > > () ; if filters . is_empty () { None } else { Some (filters) } }
    };
}

parse_include!();