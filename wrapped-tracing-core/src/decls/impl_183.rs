macro_rules! deps {
    () => {
        ValueSet!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl fmt :: Display for ValueSet < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . values . iter () . fold (& mut f . debug_map () , | dbg , (key , v) | { if let Some (val) = v { val . record (key , dbg) ; } dbg }) . finish () } }
    };
}

impl_183!()