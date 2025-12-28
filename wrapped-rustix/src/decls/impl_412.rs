macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! impl_412 {
    () => {
        deps!();
        impl Default for io_uring_sync_cancel_reg { # [inline] fn default () -> Self { Self { addr : Default :: default () , fd : Default :: default () , flags : Default :: default () , timeout : Timespec { tv_sec : 0 , tv_nsec : 0 , } , opcode : Default :: default () , pad : Default :: default () , pad2 : Default :: default () , } } }
    };
}

impl_412!();