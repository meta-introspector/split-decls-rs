// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl CargoToml { pub fn version (& self) -> Result < & str > { self . get ("version") } fn get (& self , field : & str) -> Result < & str > { for line in self . contents . lines () { let words = line . split_ascii_whitespace () . collect :: < Vec < _ > > () ; match words . as_slice () { [n , "=" , v , ..] if n . trim () == field => { assert ! (v . starts_with ('"') && v . ends_with ('"')) ; return Ok (& v [1 .. v . len () - 1]) ; } _ => () , } } Err (anyhow ! ("can't find `{}` in {}" , field , self . path . display ())) ? } pub fn publish (& self , sh : & mut Shell) -> Result < () > { let token = env :: var ("CRATES_IO_TOKEN") . unwrap_or ("no token" . to_string ()) ; let dry_run = dry_run () ; cmd ! (sh , "cargo publish --token {token} {dry_run...}") . run () ? ; Ok (()) } pub fn publish_all (& self , dirs : & [& str] , sh : & mut Shell) -> Result < () > { let token = env :: var ("CRATES_IO_TOKEN") . unwrap_or ("no token" . to_string ()) ; if dry_run () . is_none () { for & dir in dirs { for _ in 0 .. 20 { std :: thread :: sleep (Duration :: from_secs (10)) ; if cmd ! (sh , "cargo publish --manifest-path {dir}'/Cargo.toml' --token {token} --dry-run") . run () . is_ok () { break ; } } cmd ! (sh , "cargo publish --manifest-path {dir}'/Cargo.toml' --token {token}") . run () ? ; } } Ok (()) } }
};
}
