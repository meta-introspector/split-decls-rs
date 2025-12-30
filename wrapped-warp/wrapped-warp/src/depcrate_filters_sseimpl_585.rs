// Generated macro for impl_585 (impl)
macro_rules! Depcrate_filters_sseimpl_585 {
() => {
// Module: crate::filters::sse
// Provides: {"impl_585"}
// Dependencies: {}
impl fmt :: Display for Event { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (ref comment) = & self . comment { ":" . fmt (f) ? ; comment . fmt (f) ? ; f . write_char ('\n') ? ; } if let Some (ref event) = & self . event { "event:" . fmt (f) ? ; event . fmt (f) ? ; f . write_char ('\n') ? ; } match self . data { Some (DataType :: Text (ref data)) => { for line in data . split ('\n') { "data:" . fmt (f) ? ; line . fmt (f) ? ; f . write_char ('\n') ? ; } } Some (DataType :: Json (ref data)) => { "data:" . fmt (f) ? ; data . fmt (f) ? ; f . write_char ('\n') ? ; } None => { } } if let Some (ref id) = & self . id { "id:" . fmt (f) ? ; id . fmt (f) ? ; f . write_char ('\n') ? ; } if let Some (ref duration) = & self . retry { "retry:" . fmt (f) ? ; let secs = duration . as_secs () ; let millis = duration . subsec_millis () ; if secs > 0 { secs . fmt (f) ? ; if millis < 10 { f . write_str ("00") ? ; } else if millis < 100 { f . write_char ('0') ? ; } } millis . fmt (f) ? ; f . write_char ('\n') ? ; } f . write_char ('\n') ? ; Ok (()) } }
};
}
