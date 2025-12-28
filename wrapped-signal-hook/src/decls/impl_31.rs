macro_rules! deps {
    () => {
        WithOrigin!();
        Origin!();
        Exfiltrator!();
        Slot!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        unsafe impl Exfiltrator for WithOrigin { type Storage = Slot ; type Output = Origin ; fn supports_signal (& self , signal : c_int) -> bool { self . 0 . supports_signal (signal) } fn store (& self , slot : & Slot , signal : c_int , info : & siginfo_t) { self . 0 . store (slot , signal , info) } fn load (& self , slot : & Self :: Storage , signal : c_int) -> Option < Origin > { self . 0 . load (slot , signal) . map (| info | unsafe { Origin :: extract (& info) }) } fn init (& self , slot : & Self :: Storage , signal : c_int) { self . 0 . init (slot , signal) } }
    };
}

impl_31!()