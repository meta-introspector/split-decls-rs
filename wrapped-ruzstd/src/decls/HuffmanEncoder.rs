macro_rules! deps {
    () => {
        HuffmanTable!();
        BitWriter!();
    };
}

macro_rules! HuffmanEncoder {
    () => {
        deps!();
        pub (crate) struct HuffmanEncoder < 'output , 'table , V : AsMut < Vec < u8 > > > { table : & 'table HuffmanTable , writer : & 'output mut BitWriter < V > , }
    };
}

HuffmanEncoder!()