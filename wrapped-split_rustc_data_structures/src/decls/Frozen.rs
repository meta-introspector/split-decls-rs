macro_rules! Frozen {
    () => {
        # [doc = " An owned immutable value."] # [derive (Debug , Clone)] pub struct Frozen < T > (T) ;
    };
}

Frozen!()