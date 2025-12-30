// Generated macro for zip (function)
macro_rules! Depcrate_distzip {
() => {
// Module: crate::dist
// Provides: {"zip"}
// Dependencies: {}
fn zip (src_path : & Path , symbols_path : Option < & PathBuf > , dest_path : & Path) -> anyhow :: Result < () > { let file = File :: create (dest_path) ? ; let mut writer = ZipWriter :: new (BufWriter :: new (file)) ; writer . start_file (src_path . file_name () . unwrap () . to_str () . unwrap () , SimpleFileOptions :: default () . last_modified_time (DateTime :: try_from (OffsetDateTime :: from (std :: fs :: metadata (src_path) ? . modified () ?)) . unwrap () ,) . unix_permissions (0o755) . compression_method (zip :: CompressionMethod :: Deflated) . compression_level (Some (9)) ,) ? ; let mut input = io :: BufReader :: new (File :: open (src_path) ?) ; io :: copy (& mut input , & mut writer) ? ; if let Some (symbols_path) = symbols_path { writer . start_file (symbols_path . file_name () . unwrap () . to_str () . unwrap () , SimpleFileOptions :: default () . last_modified_time (DateTime :: try_from (OffsetDateTime :: from (std :: fs :: metadata (src_path) ? . modified () ? ,)) . unwrap () ,) . compression_method (zip :: CompressionMethod :: Deflated) . compression_level (Some (9)) ,) ? ; let mut input = io :: BufReader :: new (File :: open (symbols_path) ?) ; io :: copy (& mut input , & mut writer) ? ; } writer . finish () ? ; Ok (()) }
};
}
