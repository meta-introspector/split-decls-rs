macro_rules! deps {
    () => {
        Buffers!();
    };
}

macro_rules! FmtEvent {
    () => {
        deps!();
        pub struct FmtEvent < 'a > { pub bufs : & 'a mut Buffers , pub comma : bool , }
    };
}

FmtEvent!()