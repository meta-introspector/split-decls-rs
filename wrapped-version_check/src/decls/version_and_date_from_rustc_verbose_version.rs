macro_rules! version_and_date_from_rustc_verbose_version {
    () => {
        # [doc = " Parses (version, date) as available from rustc verbose version output."] fn version_and_date_from_rustc_verbose_version (s : & str) -> (Option < String > , Option < String >) { let (mut version , mut date) = (None , None) ; for line in s . lines () { let split = | s : & str | s . splitn (2 , ":") . nth (1) . map (| s | s . trim () . to_string ()) ; match line . trim () . split (" ") . nth (0) { Some ("rustc") => { let (v , d) = version_and_date_from_rustc_version (line) ; version = version . or (v) ; date = date . or (d) ; } Some ("release:") => version = split (line) , Some ("commit-date:") if line . ends_with ("unknown") => date = None , Some ("commit-date:") => date = split (line) , _ => continue , } } (version , date) }
    };
}

version_and_date_from_rustc_verbose_version!()