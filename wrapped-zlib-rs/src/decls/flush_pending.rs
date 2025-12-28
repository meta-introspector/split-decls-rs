macro_rules! deps {
    () => {
        DeflateStream!();
    };
}

macro_rules! flush_pending {
    () => {
        deps!();
        pub (crate) fn flush_pending (stream : & mut DeflateStream) { let state = & mut stream . state ; state . bit_writer . flush_bits () ; let pending = state . bit_writer . pending . pending () ; let len = Ord :: min (pending . len () , stream . avail_out as usize) ; if len == 0 { return ; } trace ! ("\n[FLUSH {len} bytes]") ; unsafe { core :: ptr :: copy_nonoverlapping (pending . as_ptr () , stream . next_out , len) } ; stream . next_out = stream . next_out . wrapping_add (len) ; stream . total_out += len as crate :: c_api :: z_size ; stream . avail_out -= len as crate :: c_api :: uInt ; state . bit_writer . pending . advance (len) ; }
    };
}

flush_pending!()