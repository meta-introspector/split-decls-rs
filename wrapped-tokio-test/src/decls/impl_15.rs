macro_rules! deps {
    () => {
        PanicMsgSnippet!();
        Mock!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Mock { fn pmsg (& self) -> PanicMsgSnippet < '_ > { PanicMsgSnippet (& self . inner) } }
    };
}

impl_15!()