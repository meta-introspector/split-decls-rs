// Generated macro for write_style_timestamp (function)
macro_rules! Depcrate_timewrite_style_timestamp {
() => {
// Module: crate::time
// Provides: {"write_style_timestamp"}
// Dependencies: {}
fn write_style_timestamp (ansi : bool , timestamp : String , unit : & str , w : & mut impl Write ,) -> std :: fmt :: Result { write ! (w , "{timestamp}{unit}" , timestamp = styled (ansi , Style :: new () . dimmed () , timestamp) , unit = styled (ansi , Style :: new () . dimmed () , unit) ,) }
};
}
