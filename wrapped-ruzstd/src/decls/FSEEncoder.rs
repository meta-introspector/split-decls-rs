macro_rules! deps {
    () => {
        BitWriter!();
        FSETable!();
    };
}

macro_rules! FSEEncoder {
    () => {
        deps!();
        pub (crate) struct FSEEncoder < 'output , V : AsMut < Vec < u8 > > > { pub (super) table : FSETable , writer : & 'output mut BitWriter < V > , }
    };
}

FSEEncoder!();