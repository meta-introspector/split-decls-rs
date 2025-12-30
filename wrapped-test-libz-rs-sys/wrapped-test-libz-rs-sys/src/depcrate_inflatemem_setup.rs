// Generated macro for mem_setup (function)
macro_rules! Depcrate_inflatemem_setup {
() => {
// Module: crate::inflate
// Provides: {"mem_setup"}
// Dependencies: {}
fn mem_setup () -> libz_rs_sys :: z_stream { let zone = MemZone { items : Vec :: new () , total : 0 , highwater : 0 , limit : 0 , not_lifo : 0 , rogue : 0 , } ; let zone = Box :: new (zone) ; let stream = libz_rs_sys :: z_stream { next_in : std :: ptr :: null_mut () , avail_in : 0 , total_in : 0 , next_out : std :: ptr :: null_mut () , avail_out : 0 , total_out : 0 , msg : std :: ptr :: null_mut () , state : std :: ptr :: null_mut () , zalloc : Some (mem_alloc) , zfree : Some (mem_free) , opaque : Box :: leak (zone) as * mut _ as * mut c_void , data_type : 0 , adler : 0 , reserved : 0 , } ; stream }
};
}
