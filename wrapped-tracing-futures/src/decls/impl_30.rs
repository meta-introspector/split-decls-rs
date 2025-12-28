macro_rules! deps {
    () => {
        WithDispatch!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [cfg (all (feature = "std-future" , feature = "std"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "std-future" , feature = "std"))))] impl < T : crate :: stdlib :: future :: Future > crate :: stdlib :: future :: Future for WithDispatch < T > { type Output = T :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> crate :: stdlib :: task :: Poll < Self :: Output > { let this = self . project () ; let dispatch = this . dispatch ; let future = this . inner ; dispatcher :: with_default (dispatch , | | future . poll (cx)) } }
    };
}

impl_30!()