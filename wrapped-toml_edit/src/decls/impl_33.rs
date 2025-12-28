macro_rules! deps {
    () => {
        DocumentMut!();
        Document!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < S : AsRef < str > > Document < S > { # [doc = " Allow editing of the [`DocumentMut`]"] pub fn into_mut (mut self) -> DocumentMut { self . despan () ; DocumentMut { root : self . root , trailing : self . trailing , } } }
    };
}

impl_33!();