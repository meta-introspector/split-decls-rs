macro_rules! deps {
    () => {
        TempDir!();
    };
}

macro_rules! create {
    () => {
        deps!();
        pub fn create (path : PathBuf , permissions : Option < & std :: fs :: Permissions > , disable_cleanup : bool ,) -> io :: Result < TempDir > { if permissions . map_or (false , | p | p . readonly ()) { return not_supported ("changing permissions is not supported on this platform") ; } fs :: create_dir (& path) . with_err_path (| | & path) . map (| _ | TempDir { path : path . into_boxed_path () , disable_cleanup , }) }
    };
}

create!()