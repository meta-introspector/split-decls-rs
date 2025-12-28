macro_rules! get_version_and_date {
    () => {
        # [doc = " Returns (version, date) as available from `rustc --version`."] fn get_version_and_date () -> Option < (Option < String > , Option < String >) > { let rustc = env :: var ("RUSTC") . unwrap_or_else (| _ | "rustc" . to_string ()) ; Command :: new (rustc) . arg ("--verbose") . arg ("--version") . output () . ok () . and_then (| output | String :: from_utf8 (output . stdout) . ok ()) . map (| s | version_and_date_from_rustc_verbose_version (& s)) }
    };
}

get_version_and_date!()