// Generated macro for impl_67 (impl)
macro_rules! Depcrate_scannerimpl_67 {
() => {
// Module: crate::scanner
// Provides: {"impl_67"}
// Dependencies: {}
impl fmt :: Display for ScanError { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "{} at byte {} line {} column {}" , self . info , self . mark . index , self . mark . line , self . mark . col + 1 ,) } }
};
}
