// Generated macro for roundtrip (function)
macro_rules! Depcrate_fseroundtrip {
() => {
// Module: crate::fse
// Provides: {"roundtrip"}
// Dependencies: {}
# [test] fn roundtrip () { round_trip (& (0 .. 64) . collect :: < alloc :: vec :: Vec < _ > > ()) ; let mut data = alloc :: vec ! [] ; data . extend (0 .. 32) ; data . extend (0 .. 32) ; data . extend (0 .. 32) ; data . extend (0 .. 32) ; data . extend (0 .. 32) ; data . extend (20 .. 32) ; data . extend (20 .. 32) ; data . extend (0 .. 32) ; data . extend (20 .. 32) ; data . extend (100 .. 255) ; data . extend (20 .. 32) ; data . extend (20 .. 32) ; round_trip (& data) ; # [cfg (feature = "std")] if std :: fs :: exists ("fuzz/artifacts/fse") . unwrap_or (false) { for file in std :: fs :: read_dir ("fuzz/artifacts/fse") . unwrap () { if file . as_ref () . unwrap () . file_type () . unwrap () . is_file () { let data = std :: fs :: read (file . unwrap () . path ()) . unwrap () ; round_trip (& data) ; } } } }
};
}
