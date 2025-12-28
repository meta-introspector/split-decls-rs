macro_rules! SerializeLevel {
    () => {
        # [derive (Debug)] pub struct SerializeLevel < 'a > (& 'a Level) ;
    };
}

SerializeLevel!()