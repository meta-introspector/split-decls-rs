// Generated macro for impl_1849 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1849 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1849"}
// Dependencies: {}
impl BorrowedHandle < '_ > { # [doc = " Creates a new `OwnedHandle` instance that shares the same underlying"] # [doc = " object as the existing `BorrowedHandle` instance."] # [stable (feature = "io_safety" , since = "1.63.0")] pub fn try_clone_to_owned (& self) -> crate :: io :: Result < OwnedHandle > { self . duplicate (0 , false , sys :: c :: DUPLICATE_SAME_ACCESS) } pub (crate) fn duplicate (& self , access : u32 , inherit : bool , options : u32 ,) -> io :: Result < OwnedHandle > { let handle = self . as_raw_handle () ; if handle . is_null () { return unsafe { Ok (OwnedHandle :: from_raw_handle (handle)) } ; } let mut ret = ptr :: null_mut () ; cvt (unsafe { let cur_proc = sys :: c :: GetCurrentProcess () ; sys :: c :: DuplicateHandle (cur_proc , handle , cur_proc , & mut ret , access , inherit as sys :: c :: BOOL , options ,) }) ? ; unsafe { Ok (OwnedHandle :: from_raw_handle (ret)) } } }
};
}
