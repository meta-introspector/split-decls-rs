macro_rules! deps {
    () => {
        SignalsInfo!();
        Forever!();
        Exfiltrator!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'a , E : Exfiltrator > IntoIterator for & 'a mut SignalsInfo < E > { type Item = E :: Output ; type IntoIter = Forever < 'a , E > ; fn into_iter (self) -> Self :: IntoIter { self . forever () } }
    };
}

impl_47!()