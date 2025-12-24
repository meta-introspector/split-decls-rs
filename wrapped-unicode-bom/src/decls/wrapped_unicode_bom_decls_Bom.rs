use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Unicode byte-order mark (BOM) abstraction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bom {
    /// Indicates no BOM was detected.
    Null,
    /// Indicates [BOCU-1](https://www.unicode.org/notes/tn6/) BOM was detected.
    Bocu1,
    /// Indicates [GB 18030](https://en.wikipedia.org/wiki/GB_18030) BOM was detected.
    Gb18030,
    /// Indicates [SCSU](https://www.unicode.org/reports/tr6/) BOM was detected.
    Scsu,
    /// Indicates [UTF-EBCIDC](https://www.unicode.org/reports/tr16/) BOM was detected.
    UtfEbcdic,
    /// Indicates [UTF-1](https://en.wikipedia.org/wiki/UTF-1) BOM was detected.
    Utf1,
    /// Indicates [UTF-7](https://tools.ietf.org/html/rfc2152) BOM was detected.
    Utf7,
    /// Indicates [UTF-8](https://tools.ietf.org/html/rfc3629) BOM was detected.
    Utf8,
    /// Indicates [UTF-16](https://tools.ietf.org/html/rfc2781) (big-endian) BOM was detected.
    Utf16Be,
    /// Indicates [UTF-16](https://tools.ietf.org/html/rfc2781) (little-endian) BOM was detected.
    Utf16Le,
    /// Indicates [UTF-32](https://www.unicode.org/reports/tr19/) (big-endian) BOM was detected.
    Utf32Be,
    /// Indicates [UTF-32](https://www.unicode.org/reports/tr19/) (little-endian) BOM was detected.
    Utf32Le,
}
