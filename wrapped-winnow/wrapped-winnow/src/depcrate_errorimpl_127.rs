// Generated macro for impl_127 (impl)
macro_rules! Depcrate_errorimpl_127 {
() => {
// Module: crate::error
// Provides: {"impl_127"}
// Dependencies: {}
# [cfg (feature = "std")] impl < I , C > TreeError < I , C > where I : core :: fmt :: Display , C : fmt :: Display , { fn write (& self , f : & mut fmt :: Formatter < '_ > , indent : usize) -> fmt :: Result { let child_indent = indent + 2 ; match self { TreeError :: Base (base) => { writeln ! (f , "{:indent$}{base}" , "") ? ; } TreeError :: Stack { base , stack } => { base . write (f , indent) ? ; for (level , frame) in stack . iter () . enumerate () { match frame { TreeErrorFrame :: Kind (frame) => { writeln ! (f , "{:child_indent$}{level}: {frame}" , "") ? ; } TreeErrorFrame :: Context (frame) => { writeln ! (f , "{:child_indent$}{level}: {frame}" , "") ? ; } } } } TreeError :: Alt (alt) => { writeln ! (f , "{:indent$}during one of:" , "") ? ; for child in alt { child . write (f , child_indent) ? ; } } } Ok (()) } }
};
}
