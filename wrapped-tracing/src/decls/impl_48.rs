macro_rules! impl_48 {
    () => {
        # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl < T : Future > Future for WithDispatch < T > { type Output = T :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let dispatcher = this . dispatcher ; let future = this . inner ; let _default = dispatcher :: set_default (dispatcher) ; future . poll (cx) } }
    };
}

impl_48!();