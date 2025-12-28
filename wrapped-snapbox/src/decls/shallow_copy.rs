macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! shallow_copy {
    () => {
        deps!();
        # [doc = " Copy a file system entry, without recursing"] pub (crate) fn shallow_copy (source : & std :: path :: Path , dest : & std :: path :: Path ,) -> Result < () , crate :: assert :: Error > { let meta = source . symlink_metadata () . map_err (| e | format ! ("Failed to read metadata from {}: {}" , source . display () , e)) ? ; if meta . is_dir () { std :: fs :: create_dir_all (dest) . map_err (| e | format ! ("Failed to create {}: {}" , dest . display () , e)) ? ; } else if meta . is_file () { std :: fs :: copy (source , dest) . map_err (| e | { format ! ("Failed to copy {} to {}: {}" , source . display () , dest . display () , e) }) ? ; copy_stats (& meta , dest) . map_err (| e | { format ! ("Failed to copy {} metadata to {}: {}" , source . display () , dest . display () , e) }) ? ; } else if let Ok (target) = std :: fs :: read_link (source) { symlink_to_file (dest , & target) . map_err (| e | format ! ("Failed to create symlink {}: {}" , dest . display () , e)) ? ; } Ok (()) }
    };
}

shallow_copy!()