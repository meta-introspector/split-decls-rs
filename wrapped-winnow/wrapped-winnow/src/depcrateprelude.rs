// Generated macro for prelude (module)
macro_rules! Depcrateprelude {
() => {
// Module: crate
// Provides: {"prelude"}
// Dependencies: {}
# [doc = " Core concepts available for glob import"] # [doc = ""] # [doc = " Including"] # [doc = " - [`StreamIsPartial`][crate::stream::StreamIsPartial]"] # [doc = " - [`Parser`]"] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use winnow::prelude::*;"] # [doc = ""] # [doc = " fn parse_data(input: &mut &str) -> ModalResult<u64> {"] # [doc = "     // ..."] # [doc = " #   winnow::ascii::dec_uint(input)"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "   let result = parse_data.parse(\"100\");"] # [doc = "   assert_eq!(result, Ok(100));"] # [doc = " }"] # [doc = " ```"] pub mod prelude { pub use crate :: error :: ModalError as _ ; pub use crate :: error :: ParserError as _ ; pub use crate :: stream :: AsChar as _ ; pub use crate :: stream :: ContainsToken as _ ; pub use crate :: stream :: Stream as _ ; pub use crate :: stream :: StreamIsPartial as _ ; pub use crate :: ModalParser ; pub use crate :: ModalResult ; pub use crate :: Parser ; # [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] pub use crate :: RecoverableParser as _ ; # [cfg (test)] pub (crate) use crate :: TestResult ; }
};
}
