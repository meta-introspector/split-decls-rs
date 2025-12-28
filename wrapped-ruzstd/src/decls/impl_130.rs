macro_rules! deps {
    () => {
        StreamingDecoder!();
        FrameDecoder!();
        FrameDecoderError!();
        Read!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < READ : Read > StreamingDecoder < READ , FrameDecoder > { pub fn new (mut source : READ ,) -> Result < StreamingDecoder < READ , FrameDecoder > , FrameDecoderError > { let mut decoder = FrameDecoder :: new () ; decoder . init (& mut source) ? ; Ok (StreamingDecoder { decoder , source }) } }
    };
}

impl_130!()