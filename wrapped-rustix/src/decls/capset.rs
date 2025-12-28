macro_rules! deps {
    () => {
        CapabilitySets!();
        Pid!();
        Result!();
    };
}

macro_rules! capset {
    () => {
        deps!();
        # [inline] fn capset (pid : Option < Pid > , sets : CapabilitySets) -> io :: Result < () > { let mut header = linux_raw_sys :: general :: __user_cap_header_struct { version : linux_raw_sys :: general :: _LINUX_CAPABILITY_VERSION_3 , pid : Pid :: as_raw (pid) as backend :: c :: c_int , } ; let data = [linux_raw_sys :: general :: __user_cap_data_struct { effective : sets . effective . bits () as u32 , permitted : sets . permitted . bits () as u32 , inheritable : sets . inheritable . bits () as u32 , } , linux_raw_sys :: general :: __user_cap_data_struct { effective : (sets . effective . bits () >> u32 :: BITS) as u32 , permitted : (sets . permitted . bits () >> u32 :: BITS) as u32 , inheritable : (sets . inheritable . bits () >> u32 :: BITS) as u32 , } ,] ; backend :: thread :: syscalls :: capset (& mut header , & data) }
    };
}

capset!();