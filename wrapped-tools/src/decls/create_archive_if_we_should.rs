macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! create_archive_if_we_should {
    () => {
        deps!();
        # [doc = " The `script_identity` will be baked into the soon to be created `archive` as it identifies the script"] # [doc = " that created the contents of `source_dir`."] fn create_archive_if_we_should (source_dir : & Path , archive : & Path , script_identity : u32) -> std :: io :: Result < () > { if should_skip_all_archive_creation () || is_excluded (archive) { return Ok (()) ; } if is_lfs_pointer_file (archive) { eprintln ! ("Refusing to overwrite `gix-lfs` pointer file at \"{}\" - git lfs might not be properly installed." , archive . display ()) ; return Ok (()) ; } std :: fs :: create_dir_all (archive . parent () . expect ("archive is a file")) ? ; let meta_dir = populate_meta_dir (source_dir , script_identity) ? ; let res = (move | | { let mut buf = Vec :: < u8 > :: new () ; { let mut ar = tar :: Builder :: new (& mut buf) ; ar . mode (tar :: HeaderMode :: Deterministic) ; ar . follow_symlinks (false) ; ar . append_dir_all ("." , source_dir) ? ; ar . finish () ? ; } # [cfg_attr (feature = "xz" , allow (unused_mut))] let mut archive = std :: fs :: OpenOptions :: new () . write (true) . create (true) . truncate (true) . open (archive) ? ; # [cfg (feature = "xz")] { let mut xz_write = xz2 :: write :: XzEncoder :: new (archive , 3) ; std :: io :: copy (& mut & * buf , & mut xz_write) ? ; xz_write . finish () ? . close () } # [cfg (not (feature = "xz"))] { use std :: io :: Write ; archive . write_all (& buf) ? ; archive . close () } }) () ; # [cfg (not (windows))] std :: fs :: remove_dir_all (meta_dir) ? ; # [cfg (windows)] std :: fs :: remove_dir_all (meta_dir) . ok () ; res }
    };
}

create_archive_if_we_should!();