macro_rules! deps {
    () => {
        TempPath!();
        NamedTempFile!();
    };
}

macro_rules! create_named {
    () => {
        deps!();
        pub (crate) fn create_named (path : PathBuf , open_options : & mut OpenOptions , permissions : Option < & std :: fs :: Permissions > , keep : bool ,) -> io :: Result < NamedTempFile > { imp :: create_named (& path , open_options , permissions) . with_err_path (| | path . clone ()) . map (| file | NamedTempFile { path : TempPath { path : path . into_boxed_path () , disable_cleanup : keep , } , file , }) }
    };
}

create_named!();