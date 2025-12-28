macro_rules! deps {
    () => {
        FrameDecoder!();
        StreamingDecoder!();
        Read!();
        FrameDecoderError!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < READ : Read , DEC : BorrowMut < FrameDecoder > > StreamingDecoder < READ , DEC > { pub fn new_with_decoder (mut source : READ , mut decoder : DEC ,) -> Result < StreamingDecoder < READ , DEC > , FrameDecoderError > { decoder . borrow_mut () . init (& mut source) ? ; Ok (StreamingDecoder { decoder , source }) } }
    };
}

impl_129!()