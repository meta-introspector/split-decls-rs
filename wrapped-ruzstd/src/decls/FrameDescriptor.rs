macro_rules! FrameDescriptor {
    () => {
        # [doc = " The first byte is called the `Frame Header Descriptor`, and it describes what other fields"] # [doc = " are present."] pub struct FrameDescriptor (pub u8) ;
    };
}

FrameDescriptor!()