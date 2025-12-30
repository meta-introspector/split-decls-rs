// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_254",
decl_type: "function",
source_file: "./src/syn_cache.rs",
source_crate: ".",
deps: ["FileCache", "SynCache"],
uses: ["FileCache", "SynCache", "UNIX_EPOCH", "HashMap", "Result", "SystemTime", "None", "String", "Path", "Ok", "Some"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        FileCache!();
        SynCache!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        impl SynCache { pub fn new (cache_file : & str) -> Self { let mut cache = Self { cache : HashMap :: new () , cache_file : cache_file . to_string () , } ; let _ = cache . load () ; cache } pub fn get_cached_content (& mut self , path : & Path) -> Result < String > { let path_str = path . to_string_lossy () . to_string () ; let metadata = fs :: metadata (path) ? ; let modified = metadata . modified () ? . duration_since (SystemTime :: UNIX_EPOCH) ? . as_secs () ; if let Some (cached) = self . cache . get (& path_str) { if cached . modified_time == modified { return Ok (cached . content . clone ()) ; } } let content = fs :: read_to_string (path) ? ; self . cache . insert (path_str , FileCache { content : content . clone () , modified_time : modified , parsed_ast : None , }) ; Ok (content) } pub fn save (& self) -> Result < () > { let json = serde_json :: to_string_pretty (& self . cache) ? ; fs :: write (& self . cache_file , json) ? ; Ok (()) } fn load (& mut self) -> Result < () > { if let Ok (content) = fs :: read_to_string (& self . cache_file) { self . cache = serde_json :: from_str (& content) ? ; } Ok (()) } }
    };
}

impl_254!();