macro_rules! deps {
    () => {
        VfsPath!();
    };
}

macro_rules! windows_paths {
    () => {
        deps!();
        # [cfg (windows)] mod windows_paths { pub (crate) trait Encode { fn encode (& self , buf : & mut Vec < u8 >) ; } impl Encode for std :: ffi :: OsStr { fn encode (& self , buf : & mut Vec < u8 >) { use std :: os :: windows :: ffi :: OsStrExt ; for wchar in self . encode_wide () { buf . extend (wchar . to_le_bytes () . iter () . copied ()) ; } } } impl Encode for u8 { fn encode (& self , buf : & mut Vec < u8 >) { let wide = * self as u16 ; buf . extend (wide . to_le_bytes () . iter () . copied ()) } } impl Encode for & str { fn encode (& self , buf : & mut Vec < u8 >) { debug_assert ! (self . is_ascii ()) ; for b in self . as_bytes () { b . encode (buf) } } } pub (crate) const SEP : & str = "\\" ; const VERBATIM : & str = "\\\\?\\" ; const UNC : & str = "UNC" ; const DEVICE : & str = "\\\\.\\" ; const COLON : & str = ":" ; impl Encode for std :: path :: Prefix < '_ > { fn encode (& self , buf : & mut Vec < u8 >) { match self { std :: path :: Prefix :: Verbatim (c) => { VERBATIM . encode (buf) ; c . encode (buf) ; } std :: path :: Prefix :: VerbatimUNC (server , share) => { VERBATIM . encode (buf) ; UNC . encode (buf) ; SEP . encode (buf) ; server . encode (buf) ; SEP . encode (buf) ; share . encode (buf) ; } std :: path :: Prefix :: VerbatimDisk (d) => { VERBATIM . encode (buf) ; d . encode (buf) ; COLON . encode (buf) ; } std :: path :: Prefix :: DeviceNS (device) => { DEVICE . encode (buf) ; device . encode (buf) ; } std :: path :: Prefix :: UNC (server , share) => { SEP . encode (buf) ; SEP . encode (buf) ; server . encode (buf) ; SEP . encode (buf) ; share . encode (buf) ; } std :: path :: Prefix :: Disk (d) => { d . encode (buf) ; COLON . encode (buf) ; } } } } # [test] fn paths_encoding () { test_eq ("C:/x.rs" , "c:/x.rs") ; test_eq ("C:/x/y.rs" , "C:\\x\\y.rs") ; fn test_eq (a : & str , b : & str) { let mut b1 = Vec :: new () ; let mut b2 = Vec :: new () ; vfs (a) . encode (& mut b1) ; vfs (b) . encode (& mut b2) ; assert_eq ! (b1 , b2) ; } } # [test] fn test_sep_root_dir_encoding () { let mut buf = Vec :: new () ; vfs ("C:/x/y") . encode (& mut buf) ; assert_eq ! (& buf , & [0 , 67 , 0 , 58 , 0 , 92 , 0 , 120 , 0 , 92 , 0 , 121 , 0]) } # [cfg (test)] fn vfs (str : & str) -> super :: VfsPath { use super :: { AbsPathBuf , VfsPath } ; VfsPath :: from (AbsPathBuf :: try_from (str) . unwrap ()) } }
    };
}

windows_paths!()