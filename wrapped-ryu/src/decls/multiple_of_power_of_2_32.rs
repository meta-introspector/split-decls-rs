macro_rules! multiple_of_power_of_2_32 {
    () => {
        # [cfg_attr (feature = "no-panic" , inline)] pub fn multiple_of_power_of_2_32 (value : u32 , p : u32) -> bool { (value & ((1u32 << p) - 1)) == 0 }
    };
}

multiple_of_power_of_2_32!();