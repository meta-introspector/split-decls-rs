macro_rules! deps {
    () => {
        Encoder!();
        Encodable!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < S : Encoder , T : ? Sized + PointeeSized > Encodable < S > for & T where T : Encodable < S > , { fn encode (& self , s : & mut S) { (* * self) . encode (s) } }
    };
}

impl_9!();