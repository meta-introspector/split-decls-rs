macro_rules! deps {
    () => {
        ThreadBound!();
    };
}

macro_rules! impl_731 {
    () => {
        deps!();
        impl < T > ThreadBound < T > { pub (crate) fn new (value : T) -> Self { ThreadBound { value , thread_id : thread :: current () . id () , } } pub (crate) fn get (& self) -> Option < & T > { if thread :: current () . id () == self . thread_id { Some (& self . value) } else { None } } }
    };
}

impl_731!();