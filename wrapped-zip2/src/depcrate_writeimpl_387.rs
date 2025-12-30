// Generated macro for impl_387 (impl)
macro_rules! Depcrate_writeimpl_387 {
() => {
// Module: crate::write
// Provides: {"impl_387"}
// Dependencies: {}
# [cfg (fuzzing)] impl < 'a > arbitrary :: Arbitrary < 'a > for FileOptions < 'a , ExtendedFileOptions > { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { let mut options = FullFileOptions { compression_method : CompressionMethod :: arbitrary (u) ? , compression_level : if bool :: arbitrary (u) ? { Some (u . int_in_range (0 ..= 24) ?) } else { None } , last_modified_time : DateTime :: arbitrary (u) ? , permissions : Option :: < u32 > :: arbitrary (u) ? , large_file : bool :: arbitrary (u) ? , encrypt_with : Option :: < EncryptWith > :: arbitrary (u) ? , alignment : u16 :: arbitrary (u) ? , # [cfg (feature = "deflate-zopfli")] zopfli_buffer_size : None , .. Default :: default () } ; # [cfg (feature = "deflate-zopfli")] if options . compression_method == CompressionMethod :: Deflated && bool :: arbitrary (u) ? { options . zopfli_buffer_size = Some (if bool :: arbitrary (u) ? { 2 } else { 3 } << u . int_in_range (8 ..= 20) ?) ; } u . arbitrary_loop (Some (0) , Some (10) , | u | { options . add_extra_data (u . int_in_range (2 ..= u16 :: MAX) ? , Box :: < [u8] > :: arbitrary (u) ? , bool :: arbitrary (u) ? ,) . map_err (| _ | arbitrary :: Error :: IncorrectFormat) ? ; Ok (core :: ops :: ControlFlow :: Continue (())) }) ? ; ZipWriter :: new (Cursor :: new (Vec :: new ())) . start_file ("" , options . clone ()) . map_err (| _ | arbitrary :: Error :: IncorrectFormat) ? ; Ok (options) } }
};
}
