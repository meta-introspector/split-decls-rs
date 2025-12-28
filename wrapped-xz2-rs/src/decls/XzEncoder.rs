macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! XzEncoder {
    () => {
        deps!();
        # [doc = " A compression stream which will have uncompressed data written to it and"] # [doc = " will write compressed data to an output stream."] pub struct XzEncoder < W : Write > { data : Stream , obj : Option < W > , buf : Vec < u8 > , }
    };
}

XzEncoder!()