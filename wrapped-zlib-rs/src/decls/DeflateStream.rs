macro_rules! deps {
    () => {
        Bytef!();
        Allocator!();
        State!();
    };
}

macro_rules! DeflateStream {
    () => {
        deps!();
        # [repr (C)] pub struct DeflateStream < 'a > { pub (crate) next_in : * mut crate :: c_api :: Bytef , pub (crate) avail_in : crate :: c_api :: uInt , pub (crate) total_in : crate :: c_api :: z_size , pub (crate) next_out : * mut crate :: c_api :: Bytef , pub (crate) avail_out : crate :: c_api :: uInt , pub (crate) total_out : crate :: c_api :: z_size , pub (crate) msg : * const core :: ffi :: c_char , pub (crate) state : & 'a mut State < 'a > , pub (crate) alloc : Allocator < 'a > , pub (crate) data_type : core :: ffi :: c_int , pub (crate) adler : crate :: c_api :: z_checksum , pub (crate) reserved : crate :: c_api :: uLong , }
    };
}

DeflateStream!()