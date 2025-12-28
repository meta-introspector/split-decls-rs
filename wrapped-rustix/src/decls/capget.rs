macro_rules! deps {
    () => {
        Pid!();
        Result!();
        CapabilitySets!();
    };
}

macro_rules! capget {
    () => {
        deps!();
        # [inline] # [allow (unsafe_code)] fn capget (pid : Option < Pid >) -> io :: Result < CapabilitySets > { let mut data = [MaybeUninit :: < linux_raw_sys :: general :: __user_cap_data_struct > :: uninit () ; 2] ; let data = { let mut header = linux_raw_sys :: general :: __user_cap_header_struct { version : linux_raw_sys :: general :: _LINUX_CAPABILITY_VERSION_3 , pid : Pid :: as_raw (pid) as backend :: c :: c_int , } ; backend :: thread :: syscalls :: capget (& mut header , & mut data) ? ; unsafe { (data [0] . assume_init () , data [1] . assume_init ()) } } ; let effective = u64 :: from (data . 0 . effective) | (u64 :: from (data . 1 . effective) << u32 :: BITS) ; let permitted = u64 :: from (data . 0 . permitted) | (u64 :: from (data . 1 . permitted) << u32 :: BITS) ; let inheritable = u64 :: from (data . 0 . inheritable) | (u64 :: from (data . 1 . inheritable) << u32 :: BITS) ; Ok (CapabilitySets { effective : CapabilitySet :: from_bits_retain (effective) , permitted : CapabilitySet :: from_bits_retain (permitted) , inheritable : CapabilitySet :: from_bits_retain (inheritable) , }) }
    };
}

capget!();