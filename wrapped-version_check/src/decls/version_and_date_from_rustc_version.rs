macro_rules! version_and_date_from_rustc_version {
    () => {
        # [doc = " Parses (version, date) as available from rustc version string."] fn version_and_date_from_rustc_version (s : & str) -> (Option < String > , Option < String >) { let last_line = s . lines () . last () . unwrap_or (s) ; let mut components = last_line . trim () . split (" ") ; let version = components . nth (1) ; let date = components . filter (| c | c . ends_with (')')) . next () . map (| s | { s . trim_right () . trim_right_matches (")") . trim_left () . trim_left_matches ('(') }) ; (version . map (| s | s . to_string ()) , date . map (| s | s . to_string ())) }
    };
}

version_and_date_from_rustc_version!()