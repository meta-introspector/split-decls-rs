macro_rules! deps {
    () => {
        Term!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Term { fn new () -> Self { Term { spec : ColorSpec :: new () , stream : Stream :: stderr (ColorChoice :: Auto) , start_of_line : true , } } fn set_color (& mut self , spec : & ColorSpec) { if self . spec != * spec { self . spec = spec . clone () ; self . start_of_line = true ; } } fn reset (& mut self) { self . spec = ColorSpec :: new () ; let _ = self . stream . reset () ; } }
    };
}

impl_9!()