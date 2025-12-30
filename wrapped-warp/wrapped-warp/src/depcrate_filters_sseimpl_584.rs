// Generated macro for impl_584 (impl)
macro_rules! Depcrate_filters_sseimpl_584 {
() => {
// Module: crate::filters::sse
// Provides: {"impl_584"}
// Dependencies: {}
impl Event { # [doc = " Set Server-sent event data"] # [doc = " data field(s) (\"data:<content>\")"] pub fn data < T : Into < String > > (mut self , data : T) -> Event { self . data = Some (DataType :: Text (data . into ())) ; self } # [doc = " Set Server-sent event data"] # [doc = " data field(s) (\"data:<content>\")"] pub fn json_data < T : Serialize > (mut self , data : T) -> Result < Event , Error > { self . data = Some (DataType :: Json (serde_json :: to_string (& data) ?)) ; Ok (self) } # [doc = " Set Server-sent event comment"] # [doc = " Comment field (\":<comment-text>\")"] pub fn comment < T : Into < String > > (mut self , comment : T) -> Event { self . comment = Some (comment . into ()) ; self } # [doc = " Set Server-sent event event"] # [doc = " Event name field (\"event:<event-name>\")"] pub fn event < T : Into < String > > (mut self , event : T) -> Event { self . event = Some (event . into ()) ; self } # [doc = " Set Server-sent event retry"] # [doc = " Retry timeout field (\"retry:<timeout>\")"] pub fn retry (mut self , duration : Duration) -> Event { self . retry = Some (duration) ; self } # [doc = " Set Server-sent event id"] # [doc = " Identifier field (\"id:<identifier>\")"] pub fn id < T : Into < String > > (mut self , id : T) -> Event { self . id = Some (id . into ()) ; self } }
};
}
