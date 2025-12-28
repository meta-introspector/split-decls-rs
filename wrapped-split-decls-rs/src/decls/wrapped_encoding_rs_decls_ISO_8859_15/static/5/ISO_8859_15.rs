use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
# [doc = " The ISO-8859-15 encoding."] # [doc = ""] # [doc = " This is the revised Western European part of the ISO/IEC 8859 encoding"] # [doc = " family. This encoding is also known as Latin 9."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/iso-8859-15.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-15-bmp.html)"] # [doc = ""] # [doc = " This encoding matches the Windows code page 28605."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static ISO_8859_15 : & 'static Encoding = & ISO_8859_15_INIT ;
}