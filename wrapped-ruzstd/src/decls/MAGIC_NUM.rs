macro_rules! MAGIC_NUM {
    () => {
        # [doc = " This 4 byte (little endian) magic number refers to the start of a dictionary"] pub const MAGIC_NUM : [u8 ; 4] = [0x37 , 0xA4 , 0x30 , 0xEC] ;
    };
}

MAGIC_NUM!()