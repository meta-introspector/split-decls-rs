macro_rules! deps {
    () => {
        ReturnCode!();
        State!();
        InflateStream!();
    };
}

macro_rules! copy {
    () => {
        deps!();
        pub unsafe fn copy < 'a > (dest : & mut MaybeUninit < InflateStream < 'a > > , source : & InflateStream < 'a > ,) -> ReturnCode { if source . next_out . is_null () || (source . next_in . is_null () && source . avail_in != 0) { return ReturnCode :: StreamError ; } unsafe { core :: ptr :: copy_nonoverlapping (source , dest . as_mut_ptr () , 1) ; } let Some (state_allocation) = source . alloc . allocate_raw :: < State > () else { return ReturnCode :: MemError ; } ; let state = & source . state ; let writer : MaybeUninit < Writer > = unsafe { core :: ptr :: read (& state . writer as * const _ as * const MaybeUninit < Writer >) } ; let mut copy = State { mode : state . mode , flags : state . flags , wrap : state . wrap , len_table : state . len_table , dist_table : state . dist_table , wbits : state . wbits , window : Window :: empty () , head : None , ncode : state . ncode , nlen : state . nlen , ndist : state . ndist , have : state . have , next : state . next , bit_reader : state . bit_reader , writer : Writer :: new (& mut []) , total : state . total , length : state . length , offset : state . offset , extra : state . extra , back : state . back , was : state . was , chunksize : state . chunksize , in_available : state . in_available , out_available : state . out_available , lens : state . lens , work : state . work , error_message : state . error_message , flush : state . flush , checksum : state . checksum , crc_fold : state . crc_fold , dmax : state . dmax , gzip_flags : state . gzip_flags , codes_codes : state . codes_codes , len_codes : state . len_codes , dist_codes : state . dist_codes , } ; if ! state . window . is_empty () { let Some (window) = state . window . clone_in (& source . alloc) else { unsafe { source . alloc . deallocate (state_allocation . as_ptr () , 1) } ; return ReturnCode :: MemError ; } ; copy . window = window ; } unsafe { state_allocation . as_ptr () . write (copy) } ; let field_ptr = unsafe { core :: ptr :: addr_of_mut ! ((* dest . as_mut_ptr ()) . state) } ; unsafe { core :: ptr :: write (field_ptr as * mut * mut State , state_allocation . as_ptr ()) } ; let field_ptr = unsafe { core :: ptr :: addr_of_mut ! ((* dest . as_mut_ptr ()) . state . writer) } ; unsafe { core :: ptr :: copy (writer . as_ptr () , field_ptr , 1) } ; let field_ptr = unsafe { core :: ptr :: addr_of_mut ! ((* dest . as_mut_ptr ()) . state . head) } ; unsafe { core :: ptr :: copy (& source . state . head , field_ptr , 1) } ; ReturnCode :: Ok }
    };
}

copy!()