use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Methods for determining displayed width of Unicode characters.
pub trait UnicodeWidthChar: private::Sealed {
    /// Returns the character's displayed width in columns, or `None` if the
    /// character is a control character.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 1 column wide. This is consistent with the recommendations for non-CJK
    /// contexts, or when the context cannot be reliably determined.
    fn width(self) -> Option<usize>;
    /// Returns the character's displayed width in columns, or `None` if the
    /// character is a control character.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 2 columns wide. This is consistent with the recommendations for
    /// CJK contexts.
    #[cfg(feature = "cjk")]
    fn width_cjk(self) -> Option<usize>;
}
