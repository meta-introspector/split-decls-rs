macro_rules! deps {
    () => {
        Result!();
        Walk!();
        Error!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        # [cfg (feature = "dir")] impl Iterator for Walk { type Item = Result < std :: path :: PathBuf , std :: io :: Error > ; fn next (& mut self) -> Option < Self :: Item > { while let Some (entry) = self . inner . next () . map (| e | { e . map (walkdir :: DirEntry :: into_path) . map_err (std :: io :: Error :: from) }) { if entry . as_ref () . ok () . and_then (| e | e . file_name ()) != Some (std :: ffi :: OsStr :: new (".keep")) { return Some (entry) ; } } None } }
    };
}

impl_179!()