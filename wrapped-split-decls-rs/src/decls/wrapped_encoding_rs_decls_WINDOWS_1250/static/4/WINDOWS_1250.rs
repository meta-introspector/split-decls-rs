use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
# [doc = " The windows-1250 encoding."] # [doc = ""] # [doc = " This is the Central European encoding for Windows."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/windows-1250.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1250-bmp.html)"] # [doc = ""] # [doc = " This encoding matches the Windows code page 1250."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static WINDOWS_1250 : & 'static Encoding = & WINDOWS_1250_INIT ;
}