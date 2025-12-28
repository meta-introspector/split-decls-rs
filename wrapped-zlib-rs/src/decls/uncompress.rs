macro_rules! deps {
    () => {
        ReturnCode!();
        InflateStream!();
        InflateFlush!();
        InflateConfig!();
    };
}

macro_rules! uncompress {
    () => {
        deps!();
        # [doc = " Inflates `source` into `dest`, and writes the final inflated size into `dest_len`."] pub fn uncompress < 'a > (output : & 'a mut [MaybeUninit < u8 >] , input : & [u8] , config : InflateConfig ,) -> (& 'a mut [u8] , ReturnCode) { let mut dest_len_ptr = output . len () as z_checksum ; let mut buf = [0u8] ; let mut left ; let mut len = input . len () as u64 ; let dest = if output . is_empty () { left = 1 ; buf . as_mut_ptr () } else { left = output . len () as u64 ; dest_len_ptr = 0 ; output . as_mut_ptr () as * mut u8 } ; let mut stream = z_stream { next_in : input . as_ptr () as * mut u8 , avail_in : 0 , zalloc : None , zfree : None , opaque : core :: ptr :: null_mut () , .. z_stream :: default () } ; let err = init (& mut stream , config) ; if err != ReturnCode :: Ok { return (& mut [] , err) ; } stream . next_out = dest ; stream . avail_out = 0 ; let Some (stream) = (unsafe { InflateStream :: from_stream_mut (& mut stream) }) else { return (& mut [] , ReturnCode :: StreamError) ; } ; let err = loop { if stream . avail_out == 0 { stream . avail_out = Ord :: min (left , u32 :: MAX as u64) as u32 ; left -= stream . avail_out as u64 ; } if stream . avail_in == 0 { stream . avail_in = Ord :: min (len , u32 :: MAX as u64) as u32 ; len -= stream . avail_in as u64 ; } let err = unsafe { inflate (stream , InflateFlush :: NoFlush) } ; if err != ReturnCode :: Ok { break err ; } } ; if ! output . is_empty () { dest_len_ptr = stream . total_out ; } else if stream . total_out != 0 && err == ReturnCode :: BufError { left = 1 ; } let avail_out = stream . avail_out ; end (stream) ; let ret = match err { ReturnCode :: StreamEnd => ReturnCode :: Ok , ReturnCode :: NeedDict => ReturnCode :: DataError , ReturnCode :: BufError if (left + avail_out as u64) != 0 => ReturnCode :: DataError , _ => err , } ; let output_slice = unsafe { core :: slice :: from_raw_parts_mut (output . as_mut_ptr () as * mut u8 , dest_len_ptr as usize) } ; (output_slice , ret) }
    };
}

uncompress!()