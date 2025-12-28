macro_rules! deps {
    () => {
        ImplodeDecoder!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl < R : Read > Read for ImplodeDecoder < R > { fn read (& mut self , buf : & mut [u8]) -> Result < usize > { if ! self . stream_read { self . stream_read = true ; let mut compressed_bytes = Vec :: new () ; self . compressed_reader . read_to_end (& mut compressed_bytes) ? ; self . stream . reserve (self . uncompressed_size as usize) ; hwexplode (& compressed_bytes , self . uncompressed_size as usize , self . large_wnd , self . lit_tree , false , & mut self . stream ,) ? ; } let available = self . stream . len () - self . read_pos ; let bytes_to_read = available . min (buf . len ()) ; buf [.. bytes_to_read] . copy_from_slice (& self . stream [self . read_pos .. self . read_pos + bytes_to_read]) ; self . read_pos += bytes_to_read ; Ok (bytes_to_read) } }
    };
}

impl_304!();