macro_rules! deps {
    () => {
        CpuSet!();
    };
}

macro_rules! impl_1376 {
    () => {
        deps!();
        impl PartialEq for CpuSet { fn eq (& self , other : & Self) -> bool { backend :: thread :: cpu_set :: CPU_EQUAL (& self . cpu_set , & other . cpu_set) } }
    };
}

impl_1376!()