macro_rules! deps {
    () => {
        SanitizerSet!();
    };
}

macro_rules! macro_507 {
    () => {
        deps!();
        bitflags :: bitflags ! { impl SanitizerSet : u16 { const ADDRESS = 1 << 0 ; const LEAK = 1 << 1 ; const MEMORY = 1 << 2 ; const THREAD = 1 << 3 ; const HWADDRESS = 1 << 4 ; const CFI = 1 << 5 ; const MEMTAG = 1 << 6 ; const SHADOWCALLSTACK = 1 << 7 ; const KCFI = 1 << 8 ; const KERNELADDRESS = 1 << 9 ; const SAFESTACK = 1 << 10 ; const DATAFLOW = 1 << 11 ; } }
    };
}

macro_507!()