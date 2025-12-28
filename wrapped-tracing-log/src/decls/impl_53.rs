macro_rules! deps {
    () => {
        AsTrace!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < 'a > AsTrace for log :: Record < 'a > { type Trace = Metadata < 'a > ; fn as_trace (& self) -> Self :: Trace { let cs_id = identify_callsite ! (loglevel_to_cs (self . level ()) . 0) ; Metadata :: new ("log record" , self . target () , self . level () . as_trace () , self . file () , self . line () , self . module_path () , field :: FieldSet :: new (FIELD_NAMES , cs_id) , Kind :: EVENT ,) } }
    };
}

impl_53!();