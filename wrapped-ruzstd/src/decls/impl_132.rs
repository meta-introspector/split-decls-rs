macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
        StreamingDecoder!();
        BlockDecodingStrategy!();
        Read!();
        FrameDecoder!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < READ : Read , DEC : BorrowMut < FrameDecoder > > Read for StreamingDecoder < READ , DEC > { fn read (& mut self , buf : & mut [u8]) -> Result < usize , Error > { let decoder = self . decoder . borrow_mut () ; if decoder . is_finished () && decoder . can_collect () == 0 { return Ok (0) ; } while decoder . can_collect () < buf . len () && ! decoder . is_finished () { let additional_bytes_needed = buf . len () - decoder . can_collect () ; match decoder . decode_blocks (& mut self . source , BlockDecodingStrategy :: UptoBytes (additional_bytes_needed) ,) { Ok (_) => { } Err (e) => { let err ; # [cfg (feature = "std")] { err = Error :: other (e) ; } # [cfg (not (feature = "std"))] { err = Error :: new (ErrorKind :: Other , alloc :: boxed :: Box :: new (e)) ; } return Err (err) ; } } } decoder . read (buf) } }
    };
}

impl_132!()