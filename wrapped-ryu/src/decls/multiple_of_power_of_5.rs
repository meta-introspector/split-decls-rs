macro_rules! multiple_of_power_of_5 {
    () => {
        # [cfg_attr (feature = "no-panic" , inline)] pub fn multiple_of_power_of_5 (value : u64 , p : u32) -> bool { pow5_factor (value) >= p }
    };
}

multiple_of_power_of_5!()