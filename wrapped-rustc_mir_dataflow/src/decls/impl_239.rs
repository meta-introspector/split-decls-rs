macro_rules! deps {
    () => {
        SlicePlusOne!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl < T : Copy > SlicePlusOne < '_ , T > { # [inline] fn read (& self) -> Option < T > { self . slice . first () . copied () . or (self . last) } # [inline] fn advance (& mut self) { match self . slice { [_ , remainder @ ..] => { self . slice = remainder ; } [] => self . last = None , } } }
    };
}

impl_239!()