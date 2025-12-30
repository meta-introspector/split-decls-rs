// Generated macro for impl_50 (impl)
macro_rules! Depcrate_arcimpl_50 {
() => {
// Module: crate::arc
// Provides: {"impl_50"}
// Dependencies: {}
impl < T : ? Sized + PartialOrd > PartialOrd for Arc < T > { fn partial_cmp (& self , other : & Arc < T >) -> Option < Ordering > { (* * self) . partial_cmp (& * * other) } fn lt (& self , other : & Arc < T >) -> bool { * (* self) < * (* other) } fn le (& self , other : & Arc < T >) -> bool { * (* self) <= * (* other) } fn gt (& self , other : & Arc < T >) -> bool { * (* self) > * (* other) } fn ge (& self , other : & Arc < T >) -> bool { * (* self) >= * (* other) } }
};
}
