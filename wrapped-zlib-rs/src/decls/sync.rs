macro_rules! deps {
    () => {
        InflateStream!();
        ReturnCode!();
        Mode!();
    };
}

macro_rules! sync {
    () => {
        deps!();
        pub fn sync (stream : & mut InflateStream) -> ReturnCode { let state = & mut stream . state ; if stream . avail_in == 0 && state . bit_reader . bits_in_buffer () < 8 { return ReturnCode :: BufError ; } if ! matches ! (state . mode , Mode :: Sync) { state . mode = Mode :: Sync ; let (buf , len) = state . bit_reader . start_sync_search () ; (state . have , _) = syncsearch (0 , & buf [.. len]) ; } let slice = unsafe { core :: slice :: from_raw_parts (stream . next_in , stream . avail_in as usize) } ; let len ; (state . have , len) = syncsearch (state . have , slice) ; stream . next_in = unsafe { stream . next_in . add (len) } ; stream . avail_in -= len as u32 ; stream . total_in += len as z_size ; if state . have != 4 { return ReturnCode :: DataError ; } if state . gzip_flags == - 1 { state . wrap = 0 ; } else { state . wrap &= ! 4 ; } let flags = state . gzip_flags ; let total_in = stream . total_in ; let total_out = stream . total_out ; reset (stream) ; stream . total_in = total_in ; stream . total_out = total_out ; stream . state . gzip_flags = flags ; stream . state . mode = Mode :: Type ; ReturnCode :: Ok }
    };
}

sync!();