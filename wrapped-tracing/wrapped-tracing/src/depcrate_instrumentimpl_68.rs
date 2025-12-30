// Generated macro for impl_68 (impl)
macro_rules! Depcrate_instrumentimpl_68 {
() => {
// Module: crate::instrument
// Provides: {"impl_68"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl < T : Future > Future for WithDispatch < T > { type Output = T :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let dispatcher = this . dispatcher ; let future = this . inner ; let _default = dispatcher :: set_default (dispatcher) ; future . poll (cx) } }
};
}
