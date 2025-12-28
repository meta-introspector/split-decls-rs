macro_rules! deps {
    () => {
        TraceScope!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl Drop for TraceScope { fn drop (& mut self) { let text = & self . text ; let style = self . style ; drop (self . guard . take ()) ; trace (& format ! ("< {text}") , style) ; } }
    };
}

impl_198!()