macro_rules! deps {
    () => {
        ShrinkDecoder!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl < R : Read > ShrinkDecoder < R > { pub fn new (inner : R , uncompressed_size : u64) -> Self { Self { compressed_reader : inner , uncompressed_size , stream_read : false , stream : Vec :: new () , read_pos : 0 , } } pub fn into_inner (self) -> R { self . compressed_reader } }
    };
}

impl_339!()