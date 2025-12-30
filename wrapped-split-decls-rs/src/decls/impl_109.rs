// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_109",
decl_type: "function",
source_file: "./src/bootstrap_cache.rs",
source_crate: ".",
deps: ["FileCache", "BootstrapCache"],
uses: ["Result", "Command", "FileCache", "Ok", "Option", "UNIX_EPOCH", "None", "Some", "BootstrapCache", "Path", "String"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        FileCache!();
        BootstrapCache!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl BootstrapCache { pub fn new () -> Self { Self :: default () } pub fn get_file (& mut self , path : & Path) -> Result < String > { if let Some (cached) = self . files . get (path) { return Ok (cached . content . clone ()) ; } let content = fs :: read_to_string (path) ? ; let git_hash = self . get_git_hash (path) ; let metadata = fs :: metadata (path) ? ; let last_modified = metadata . modified () ? . duration_since (std :: time :: UNIX_EPOCH) ? . as_secs () ; let cache_entry = FileCache { content : content . clone () , git_hash , last_modified , } ; self . files . insert (path . to_path_buf () , cache_entry) ; Ok (content) } fn get_git_hash (& self , path : & Path) -> Option < String > { std :: process :: Command :: new ("git") . args (& ["hash-object" , path . to_str () ?]) . output () . ok () . and_then (| output | { if output . status . success () { String :: from_utf8 (output . stdout) . ok () . map (| s | s . trim () . to_string ()) } else { None } }) } pub fn save_cache (& self , cache_path : & Path) -> Result < () > { let json = serde_json :: to_string_pretty (self) ? ; fs :: write (cache_path , json) ? ; Ok (()) } pub fn load_cache (cache_path : & Path) -> Result < Self > { if cache_path . exists () { let json = fs :: read_to_string (cache_path) ? ; Ok (serde_json :: from_str (& json) ?) } else { Ok (Self :: new ()) } } }
    };
}

impl_109!();