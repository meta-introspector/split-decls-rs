// Generated macro for syscallclippy (macro)
macro_rules! Depcratesyscallclippy {
() => {
// Module: crate
// Provides: {"syscallclippy"}
// Dependencies: {}
# [macro_export] macro_rules ! syscallclippy { (level8 , $ syscall : expr) => { format ! ("#[syscall=\"{}\"]" , $ syscall) } ; (level7 , $ syscall : expr) => { syscallclippy ! (level8 , $ syscall) } ; (level6 , $ syscall : expr) => { syscallclippy ! (level7 , $ syscall) } ; (level5 , $ syscall : expr) => { syscallclippy ! (level6 , $ syscall) } ; (level4 , $ syscall : expr) => { syscallclippy ! (level5 , $ syscall) } ; (level3 , $ syscall : expr) => { syscallclippy ! (level4 , $ syscall) } ; (level2 , $ syscall : expr) => { syscallclippy ! (level3 , $ syscall) } ; ($ syscall : expr) => { syscallclippy ! (level2 , $ syscall) } ; }
};
}
