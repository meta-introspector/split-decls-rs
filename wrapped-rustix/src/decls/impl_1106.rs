macro_rules! deps {
    () => {
        FlockOffsetType!();
        FlockType!();
        Flock!();
        Pid!();
    };
}

macro_rules! impl_1106 {
    () => {
        deps!();
        # [cfg (not (target_os = "horizon"))] impl Flock { pub (crate) const unsafe fn from_raw_unchecked (raw_fl : c :: flock) -> Self { Self { start : raw_fl . l_start as _ , length : raw_fl . l_len as _ , pid : Pid :: from_raw (raw_fl . l_pid) , typ : transmute :: < i16 , FlockType > (raw_fl . l_type) , offset_type : transmute :: < i16 , FlockOffsetType > (raw_fl . l_whence) , } } pub (crate) fn as_raw (& self) -> c :: flock { let mut f : c :: flock = unsafe { core :: mem :: zeroed () } ; f . l_start = self . start as _ ; f . l_len = self . length as _ ; f . l_pid = Pid :: as_raw (self . pid) ; f . l_type = self . typ as _ ; f . l_whence = self . offset_type as _ ; f } }
    };
}

impl_1106!()