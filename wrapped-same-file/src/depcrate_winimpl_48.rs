// Generated macro for impl_48 (impl)
macro_rules! Depcrate_winimpl_48 {
() => {
// Module: crate::win
// Provides: {"impl_48"}
// Dependencies: {}
impl Handle { pub fn from_path < P : AsRef < Path > > (p : P) -> io :: Result < Handle > { let h = winutil :: Handle :: from_path_any (p) ? ; let info = winutil :: file :: information (& h) ? ; Ok (Handle :: from_info (HandleKind :: Owned (h) , info)) } pub fn from_file (file : File) -> io :: Result < Handle > { let h = winutil :: Handle :: from_file (file) ; let info = winutil :: file :: information (& h) ? ; Ok (Handle :: from_info (HandleKind :: Owned (h) , info)) } fn from_std_handle (h : winutil :: HandleRef) -> io :: Result < Handle > { match winutil :: file :: information (& h) { Ok (info) => Ok (Handle :: from_info (HandleKind :: Borrowed (h) , info)) , Err (_) => Ok (Handle { kind : HandleKind :: Borrowed (h) , key : None }) , } } fn from_info (kind : HandleKind , info : winutil :: file :: Information ,) -> Handle { Handle { kind : kind , key : Some (Key { volume : info . volume_serial_number () , index : info . file_index () , }) , } } pub fn stdin () -> io :: Result < Handle > { Handle :: from_std_handle (winutil :: HandleRef :: stdin ()) } pub fn stdout () -> io :: Result < Handle > { Handle :: from_std_handle (winutil :: HandleRef :: stdout ()) } pub fn stderr () -> io :: Result < Handle > { Handle :: from_std_handle (winutil :: HandleRef :: stderr ()) } pub fn as_file (& self) -> & File { match self . kind { HandleKind :: Owned (ref h) => h . as_file () , HandleKind :: Borrowed (ref h) => h . as_file () , } } pub fn as_file_mut (& mut self) -> & mut File { match self . kind { HandleKind :: Owned (ref mut h) => h . as_file_mut () , HandleKind :: Borrowed (ref mut h) => h . as_file_mut () , } } }
};
}
