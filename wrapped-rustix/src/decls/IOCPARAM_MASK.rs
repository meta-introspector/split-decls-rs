macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! IOCPARAM_MASK {
    () => {
        deps!();
        pub const IOCPARAM_MASK : Opcode = 0x1FFF ;
    };
}

IOCPARAM_MASK!()