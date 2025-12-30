// Generated macro for impl_1031 (impl)
macro_rules! Depcrate_io_cursorimpl_1031 {
() => {
// Module: crate::io::cursor
// Provides: {"impl_1031"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > Clone for Cursor < T > where T : Clone , { # [inline] fn clone (& self) -> Self { Cursor { inner : self . inner . clone () , pos : self . pos } } # [inline] fn clone_from (& mut self , other : & Self) { self . inner . clone_from (& other . inner) ; self . pos = other . pos ; } }
};
}
