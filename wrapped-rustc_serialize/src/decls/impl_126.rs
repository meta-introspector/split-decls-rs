macro_rules! deps {
    () => {
        Encoder!();
        Encodable!();
        FileEncoder!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl Encodable < FileEncoder > for [u8] { fn encode (& self , e : & mut FileEncoder) { Encoder :: emit_usize (e , self . len ()) ; e . emit_raw_bytes (self) ; } }
    };
}

impl_126!();