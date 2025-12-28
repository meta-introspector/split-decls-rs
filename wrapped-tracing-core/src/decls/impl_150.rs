macro_rules! deps {
    () => {
        Metadata!();
        FieldSet!();
        Interest!();
        Field!();
        Callsite!();
        Value!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl fmt :: Debug for dyn Value { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct NullCallsite ; static NULL_CALLSITE : NullCallsite = NullCallsite ; impl crate :: callsite :: Callsite for NullCallsite { fn set_interest (& self , _ : crate :: subscriber :: Interest) { unreachable ! ("you somehow managed to register the null callsite?") } fn metadata (& self) -> & crate :: Metadata < '_ > { unreachable ! ("you somehow managed to access the null callsite?") } } static FIELD : Field = Field { i : 0 , fields : FieldSet :: new (& [] , crate :: identify_callsite ! (& NULL_CALLSITE)) , } ; let mut res = Ok (()) ; self . record (& FIELD , & mut | _ : & Field , val : & dyn fmt :: Debug | { res = write ! (f , "{:?}" , val) ; }) ; res } }
    };
}

impl_150!();