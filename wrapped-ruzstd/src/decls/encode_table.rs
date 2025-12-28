macro_rules! deps {
    () => {
        FseTableMode!();
        BitWriter!();
    };
}

macro_rules! encode_table {
    () => {
        deps!();
        fn encode_table (mode : & FseTableMode < '_ > , writer : & mut BitWriter < & mut Vec < u8 > >) { match mode { FseTableMode :: Predefined (_) => { } FseTableMode :: RepeateLast (_) => { } FseTableMode :: Encoded (table) => table . write_table (writer) , } }
    };
}

encode_table!();