macro_rules! ColorLevel {
    () => {
        pub struct ColorLevel < 'a > (pub & 'a Level) ;
    };
}

ColorLevel!()