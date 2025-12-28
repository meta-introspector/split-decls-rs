macro_rules! deps {
    () => {
        Read!();
        FrameDecoder!();
        Error!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        # [doc = " Read bytes from the decode_buffer that are no longer needed. While the frame is not yet finished"] # [doc = " this will retain window_size bytes, else it will drain it completely"] impl Read for FrameDecoder { fn read (& mut self , target : & mut [u8]) -> Result < usize , Error > { let state = match & mut self . state { None => return Ok (0) , Some (s) => s , } ; if state . frame_finished { state . decoder_scratch . buffer . read_all (target) } else { state . decoder_scratch . buffer . read (target) } } }
    };
}

impl_126!()