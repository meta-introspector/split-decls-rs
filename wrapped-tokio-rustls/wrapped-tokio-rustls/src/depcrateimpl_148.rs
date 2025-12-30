// Generated macro for impl_148 (impl)
macro_rules! Depcrateimpl_148 {
() => {
// Module: crate
// Provides: {"impl_148"}
// Dependencies: {}
impl < T > TlsStream < T > { pub fn get_ref (& self) -> (& T , & CommonState) { use TlsStream :: * ; match self { Client (io) => { let (io , session) = io . get_ref () ; (io , session) } Server (io) => { let (io , session) = io . get_ref () ; (io , session) } } } pub fn get_mut (& mut self) -> (& mut T , & mut CommonState) { use TlsStream :: * ; match self { Client (io) => { let (io , session) = io . get_mut () ; (io , & mut * session) } Server (io) => { let (io , session) = io . get_mut () ; (io , & mut * session) } } } }
};
}
