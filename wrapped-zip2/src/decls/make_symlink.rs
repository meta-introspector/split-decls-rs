macro_rules! deps {
    () => {
        ZipResult!();
    };
}

macro_rules! make_symlink {
    () => {
        deps!();
        pub (crate) fn make_symlink < T > (outpath : & Path , target : & [u8] , # [allow (unused)] existing_files : & IndexMap < Box < str > , T > ,) -> ZipResult < () > { let Ok (target_str) = std :: str :: from_utf8 (target) else { return Err (invalid ! ("Invalid UTF-8 as symlink target")) ; } ; # [cfg (not (any (unix , windows)))] { use std :: fs :: File ; let output = File :: create (outpath) ; output ? . write_all (target) ? ; } # [cfg (unix)] { std :: os :: unix :: fs :: symlink (Path :: new (& target_str) , outpath) ? ; } # [cfg (windows)] { let target = Path :: new (OsStr :: new (& target_str)) ; let target_is_dir_from_archive = existing_files . contains_key (target_str) && is_dir (target_str) ; let target_is_dir = if target_is_dir_from_archive { true } else if let Ok (meta) = std :: fs :: metadata (target) { meta . is_dir () } else { false } ; if target_is_dir { std :: os :: windows :: fs :: symlink_dir (target , outpath) ? ; } else { std :: os :: windows :: fs :: symlink_file (target , outpath) ? ; } } Ok (()) }
    };
}

make_symlink!()