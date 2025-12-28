macro_rules! ModifierInfo {
    () => {
        pub struct ModifierInfo { pub modifier : char , pub result : & 'static str , pub size : u16 , }
    };
}

ModifierInfo!()