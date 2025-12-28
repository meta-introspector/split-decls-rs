macro_rules! BaseNString {
    () => {
        pub struct BaseNString { start : usize , buf : [ascii :: Char ; 128] , }
    };
}

BaseNString!();