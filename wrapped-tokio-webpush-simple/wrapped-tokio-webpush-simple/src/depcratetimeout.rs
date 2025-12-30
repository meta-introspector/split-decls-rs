// Generated macro for timeout (function)
macro_rules! Depcratetimeout {
() => {
// Module: crate
// Provides: {"timeout"}
// Dependencies: {}
fn timeout < F > (f : F , dur : Duration , handle : & Handle) -> MyFuture < F :: Item > where F : Future < Error = Error > + 'static , { let timeout = Timeout :: new (dur , handle) . into_future () . flatten () ; Box :: new (f . select2 (timeout) . then (| res | { match res { Ok (Either :: A ((item , _timeout))) => Ok (item) , Ok (Either :: B ((() , _item))) => Err ("accept timed out" . into ()) , Err (Either :: A ((e , _timeout))) => Err (e) , Err (Either :: B ((e , _item))) => Err (e) . chain_err (| | "timeout failure") , } })) }
};
}
