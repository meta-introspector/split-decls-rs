macro_rules! deps {
    () => {
        State!();
        InflateStream!();
        InflateConfig!();
        Allocator!();
        ReturnCode!();
    };
}

macro_rules! init {
    () => {
        deps!();
        # [doc = " Initialize the stream in an inflate state"] pub fn init (stream : & mut z_stream , config : InflateConfig) -> ReturnCode { stream . msg = core :: ptr :: null_mut () ; # [cfg (feature = "rust-allocator")] if stream . zalloc . is_none () || stream . zfree . is_none () { stream . configure_default_rust_allocator () } # [cfg (feature = "c-allocator")] if stream . zalloc . is_none () || stream . zfree . is_none () { stream . configure_default_c_allocator () } if stream . zalloc . is_none () || stream . zfree . is_none () { return ReturnCode :: StreamError ; } let mut state = State :: new (& [] , Writer :: new (& mut [])) ; state . chunksize = 32 ; let alloc = Allocator { zalloc : stream . zalloc . unwrap () , zfree : stream . zfree . unwrap () , opaque : stream . opaque , _marker : PhantomData , } ; let Some (state_allocation) = alloc . allocate_raw :: < State > () else { return ReturnCode :: MemError ; } ; unsafe { state_allocation . as_ptr () . write (state) } ; stream . state = state_allocation . as_ptr () as * mut internal_state ; let ret = if let Some (stream) = unsafe { InflateStream :: from_stream_mut (stream) } { reset_with_config (stream , config) } else { ReturnCode :: StreamError } ; if ret != ReturnCode :: Ok { let ptr = stream . state ; stream . state = core :: ptr :: null_mut () ; unsafe { alloc . deallocate (ptr , 1) } ; } ret }
    };
}

init!()