macro_rules! invoke {
    () => {
        fn invoke (list : & [fn (& str) -> Option < Utf8PathBuf >] , executable : & str) -> Utf8PathBuf { list . iter () . find_map (| it | it (executable)) . unwrap_or_else (| | executable . into ()) }
    };
}

invoke!();