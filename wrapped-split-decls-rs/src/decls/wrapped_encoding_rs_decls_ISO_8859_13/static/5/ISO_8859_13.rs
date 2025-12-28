use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
# [doc = " The ISO-8859-13 encoding."] # [doc = ""] # [doc = " This is the Baltic part of the ISO/IEC 8859 encoding family. This encoding"] # [doc = " is also known as Latin 7."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/iso-8859-13.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-13-bmp.html)"] # [doc = ""] # [doc = " This encoding matches the Windows code page 28603, except Windows decodes"] # [doc = " unassigned code points to the Private Use Area of Unicode."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static ISO_8859_13 : & 'static Encoding = & ISO_8859_13_INIT ;
}