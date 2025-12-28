macro_rules! deps {
    () => {
        Signals!();
        SignalsInfo!();
        Exfiltrator!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < E > Debug for SignalsInfo < E > where E : Debug + Exfiltrator , E :: Storage : Debug , { fn fmt (& self , fmt : & mut Formatter) -> FmtResult { fmt . debug_tuple ("Signals") . field (& self . 0) . finish () } }
    };
}

impl_46!()