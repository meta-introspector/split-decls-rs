// Generated macro for impl_1159 (impl)
macro_rules! Depcrate_distimpl_1159 {
() => {
// Module: crate::dist
// Provides: {"impl_1159"}
// Dependencies: {}
impl OutputData { # [cfg (any (feature = "dist-server" , all (feature = "dist-client" , test)))] pub fn try_from_reader < R : Read > (r : R) -> io :: Result < Self > { use flate2 :: Compression ; use flate2 :: read :: ZlibEncoder as ZlibReadEncoder ; let mut compressor = ZlibReadEncoder :: new (r , Compression :: fast ()) ; let mut res = vec ! [] ; io :: copy (& mut compressor , & mut res) ? ; Ok (OutputData (res , compressor . total_in ())) } pub fn lens (& self) -> OutputDataLens { OutputDataLens { actual : self . 1 , compressed : self . 0 . len () as u64 , } } # [cfg (feature = "dist-client")] pub fn into_reader (self) -> impl Read { use flate2 :: read :: ZlibDecoder as ZlibReadDecoder ; ZlibReadDecoder :: new (io :: Cursor :: new (self . 0)) } }
};
}
