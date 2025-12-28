macro_rules! deps {
    () => {
        FrameCompressor!();
        CompressionLevel!();
        Read!();
        Write!();
    };
}

macro_rules! compress {
    () => {
        deps!();
        # [doc = " Convenience function to compress some source into a target without reusing any resources of the compressor"] # [doc = " ```rust"] # [doc = " use ruzstd::encoding::{compress, CompressionLevel};"] # [doc = " let data: &[u8] = &[0,0,0,0,0,0,0,0,0,0,0,0];"] # [doc = " let mut target = Vec::new();"] # [doc = " compress(data, &mut target, CompressionLevel::Fastest);"] # [doc = " ```"] pub fn compress < R : Read , W : Write > (source : R , target : W , level : CompressionLevel) { let mut frame_enc = FrameCompressor :: new (level) ; frame_enc . set_source (source) ; frame_enc . set_drain (target) ; frame_enc . compress () ; }
    };
}

compress!();