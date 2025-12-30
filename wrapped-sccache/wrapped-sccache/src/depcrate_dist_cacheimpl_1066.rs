// Generated macro for impl_1066 (impl)
macro_rules! Depcrate_dist_cacheimpl_1066 {
() => {
// Module: crate::dist::cache
// Provides: {"impl_1066"}
// Dependencies: {}
impl TcCache { pub fn new (cache_dir : & Path , cache_size : u64) -> Result < TcCache > { trace ! ("Using TcCache({:?}, {})" , cache_dir , cache_size) ; Ok (TcCache { inner : LruDiskCache :: new (cache_dir , cache_size) ? , }) } pub fn contains_toolchain (& self , tc : & Toolchain) -> bool { self . inner . contains_key (make_lru_key_path (& tc . archive_id)) } pub fn insert_with < F : FnOnce (fs :: File) -> io :: Result < () > > (& mut self , tc : & Toolchain , with : F ,) -> Result < () > { self . inner . insert_with (make_lru_key_path (& tc . archive_id) , with) ? ; let verified_archive_id = file_key (self . get (tc) ?) ? ; if verified_archive_id == tc . archive_id { Ok (()) } else { Err (anyhow ! ("written file does not match expected hash key")) } } pub fn get_file (& mut self , tc : & Toolchain) -> LruResult < fs :: File > { self . inner . get_file (make_lru_key_path (& tc . archive_id)) } pub fn get (& mut self , tc : & Toolchain) -> LruResult < Box < dyn ReadSeek > > { self . inner . get (make_lru_key_path (& tc . archive_id)) } pub fn len (& self) -> usize { self . inner . len () } pub fn is_empty (& self) -> bool { self . len () == 0 } pub fn remove (& mut self , tc : & Toolchain) -> LruResult < () > { self . inner . remove (make_lru_key_path (& tc . archive_id)) } # [cfg (feature = "dist-client")] fn insert_file (& mut self , path : & Path) -> Result < Toolchain > { let archive_id = path_key (path) ? ; self . inner . insert_file (make_lru_key_path (& archive_id) , path) ? ; Ok (Toolchain { archive_id }) } }
};
}
