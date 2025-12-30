// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl PinnedVersions { # [doc = " Attempts to extract pinned toolchain versions based on the current"] # [doc = " working directory."] # [doc = ""] # [doc = " `extract_from_pwd` expects to be called from a directory which is a"] # [doc = " child of a Cargo workspace. It extracts the pinned versions from the"] # [doc = " metadata of the root package."] fn extract_from_pwd () -> Result < PinnedVersions , Box < dyn Error > > { let manifest_dir = env :: var_os ("CARGO_MANIFEST_DIR") . ok_or ("CARGO_MANIFEST_DIR environment variable not set") ? . into_string () . map_err (| _ | "could not parse $CARGO_MANIFEST_DIR as UTF-8") ? ; let manifest_path = if manifest_dir . ends_with ("zerocopy-derive") { manifest_dir + "/../Cargo.toml" } else { manifest_dir + "/Cargo.toml" } ; let manifest = fs :: read_to_string (manifest_path) ? ; let manifest : toml :: map :: Map < String , toml :: Value > = toml :: from_str (& manifest) ? ; let manifest = toml :: Value :: Table (manifest) ; let extract = | keys : & [& str] | -> Result < String , String > { let mut val = & manifest ; for k in keys { val = val . get (k) . ok_or (format ! ("failed to look up path in Cargo.toml: {:?}" , keys)) ? ; } val . as_str () . map (| s | s . to_string ()) . ok_or (format ! ("expected string value for path in Cargo.toml: {:?}" , keys)) } ; let msrv = extract (& ["package" , "rust-version"]) ? ; let stable = extract (& ["package" , "metadata" , "ci" , "pinned-stable"]) ? ; let nightly = extract (& ["package" , "metadata" , "ci" , "pinned-nightly"]) ? ; Ok (PinnedVersions { msrv , stable , nightly }) } }
};
}
