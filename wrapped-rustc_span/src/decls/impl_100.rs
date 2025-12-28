macro_rules! deps {
    () => {
        StableCrateId!();
        Symbol!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl StableCrateId { # [doc = " Computes the stable ID for a crate with the given name and"] # [doc = " `-Cmetadata` arguments."] pub fn new (crate_name : Symbol , is_exe : bool , mut metadata : Vec < String > , cfg_version : & 'static str ,) -> StableCrateId { let mut hasher = StableHasher :: new () ; crate_name . as_str () . hash (& mut hasher) ; metadata . sort () ; metadata . dedup () ; hasher . write (b"metadata") ; for s in & metadata { hasher . write_usize (s . len ()) ; hasher . write (s . as_bytes ()) ; } hasher . write (if is_exe { b"exe" } else { b"lib" }) ; if let Some (val) = std :: env :: var_os ("RUSTC_FORCE_RUSTC_VERSION") { hasher . write (val . to_string_lossy () . into_owned () . as_bytes ()) } else { hasher . write (cfg_version . as_bytes ()) } StableCrateId (hasher . finish ()) } # [inline] pub fn as_u64 (self) -> u64 { self . 0 . as_u64 () } }
    };
}

impl_100!();