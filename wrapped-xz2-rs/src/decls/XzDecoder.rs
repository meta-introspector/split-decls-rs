macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! XzDecoder {
    () => {
        deps!();
        # [doc = " A compression stream which will have compressed data written to it and"] # [doc = " will write uncompressed data to an output stream."] pub struct XzDecoder < W : Write > { data : Stream , obj : Option < W > , buf : Vec < u8 > , }
    };
}

XzDecoder!();