macro_rules! impl_47 {
    () => {
        impl Default for z_stream { fn default () -> Self { let mut stream = Self { next_in : core :: ptr :: null_mut () , avail_in : 0 , total_in : 0 , next_out : core :: ptr :: null_mut () , avail_out : 0 , total_out : 0 , msg : core :: ptr :: null_mut () , state : core :: ptr :: null_mut () , zalloc : None , zfree : None , opaque : core :: ptr :: null_mut () , data_type : 0 , adler : 0 , reserved : 0 , } ; # [cfg (feature = "rust-allocator")] if stream . zalloc . is_none () || stream . zfree . is_none () { stream . configure_default_rust_allocator () } # [cfg (feature = "c-allocator")] if stream . zalloc . is_none () || stream . zfree . is_none () { stream . configure_default_c_allocator () } stream } }
    };
}

impl_47!()