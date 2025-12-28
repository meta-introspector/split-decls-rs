macro_rules! deps {
    () => {
        ImplodeDecoder!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl < R : Read > ImplodeDecoder < R > { pub fn new (inner : R , uncompressed_size : u64 , flags : u16) -> Self { let large_wnd = (flags & 2) != 0 ; let lit_tree = (flags & 4) != 0 ; ImplodeDecoder { compressed_reader : inner , uncompressed_size , stream_read : false , large_wnd , lit_tree , stream : Vec :: new () , read_pos : 0 , } } pub fn into_inner (self) -> R { self . compressed_reader } }
    };
}

impl_303!();