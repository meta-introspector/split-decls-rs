use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
# [doc = " The IBM866 encoding."] # [doc = ""] # [doc = " This the most notable one of the DOS Cyrillic code pages. It has the same"] # [doc = " box drawing characters as code page 437, so it can be used for decoding"] # [doc = " DOS-era ASCII + box drawing data."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/ibm866.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/ibm866-bmp.html)"] # [doc = ""] # [doc = " This encoding matches the Windows code page 866."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static IBM866 : & 'static Encoding = & IBM866_INIT ;
}