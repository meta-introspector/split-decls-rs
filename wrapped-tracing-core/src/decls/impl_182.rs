macro_rules! deps {
    () => {
        ValueSet!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl fmt :: Debug for ValueSet < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . values . iter () . fold (& mut f . debug_struct ("ValueSet") , | dbg , (key , v) | { if let Some (val) = v { val . record (key , dbg) ; } dbg }) . field ("callsite" , & self . callsite ()) . finish () } }
    };
}

impl_182!();