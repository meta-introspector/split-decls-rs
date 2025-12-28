macro_rules! NTSTATUS {
    () => {
        pub type NTSTATUS = i32 ;
    };
}

NTSTATUS!();