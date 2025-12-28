macro_rules! deps {
    () => {
        Mock!();
        PanicMsgSnippet!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Mock { fn pmsg (& self) -> PanicMsgSnippet < '_ > { PanicMsgSnippet (& self . inner) } }
    };
}

impl_15!();