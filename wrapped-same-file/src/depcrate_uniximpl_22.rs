// Generated macro for impl_22 (impl)
macro_rules! Depcrate_uniximpl_22 {
() => {
// Module: crate::unix
// Provides: {"impl_22"}
// Dependencies: {}
impl Handle { pub fn from_path < P : AsRef < Path > > (p : P) -> io :: Result < Handle > { Handle :: from_file (OpenOptions :: new () . read (true) . open (p) ?) } pub fn from_file (file : File) -> io :: Result < Handle > { let md = file . metadata () ? ; Ok (Handle { file : Some (file) , is_std : false , dev : md . dev () , ino : md . ino () , }) } pub fn from_std (file : File) -> io :: Result < Handle > { Handle :: from_file (file) . map (| mut h | { h . is_std = true ; h }) } pub fn stdin () -> io :: Result < Handle > { Handle :: from_std (unsafe { File :: from_raw_fd (0) }) } pub fn stdout () -> io :: Result < Handle > { Handle :: from_std (unsafe { File :: from_raw_fd (1) }) } pub fn stderr () -> io :: Result < Handle > { Handle :: from_std (unsafe { File :: from_raw_fd (2) }) } pub fn as_file (& self) -> & File { self . file . as_ref () . take () . unwrap () } pub fn as_file_mut (& mut self) -> & mut File { self . file . as_mut () . take () . unwrap () } pub fn dev (& self) -> u64 { self . dev } pub fn ino (& self) -> u64 { self . ino } }
};
}
