// Generated macro for ws (function)
macro_rules! Depcrate_filters_wsws {
() => {
// Module: crate::filters::ws
// Provides: {"ws"}
// Dependencies: {}
# [doc = " Creates a Websocket Filter."] # [doc = ""] # [doc = " The yielded `Ws` is used to finish the websocket upgrade."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This filter combines multiple filters internally, so you don't need them:"] # [doc = ""] # [doc = " - Method must be `GET`"] # [doc = " - Header `connection` must be `upgrade`"] # [doc = " - Header `upgrade` must be `websocket`"] # [doc = " - Header `sec-websocket-version` must be `13`"] # [doc = " - Header `sec-websocket-key` must be set."] # [doc = ""] # [doc = " If the filters are met, yields a `Ws`. Calling `Ws::on_upgrade` will"] # [doc = " return a reply with:"] # [doc = ""] # [doc = " - Status of `101 Switching Protocols`"] # [doc = " - Header `connection: upgrade`"] # [doc = " - Header `upgrade: websocket`"] # [doc = " - Header `sec-websocket-accept` with the hash value of the received key."] pub fn ws () -> impl Filter < Extract = One < Ws > , Error = Rejection > + Copy { let connection_has_upgrade = header :: header2 () . and_then (| conn : :: headers :: Connection | { if conn . contains ("upgrade") { future :: ok (()) } else { future :: err (crate :: reject :: known (MissingConnectionUpgrade)) } }) . untuple_one () ; crate :: get () . and (connection_has_upgrade) . and (header :: exact_ignore_case ("upgrade" , "websocket")) . and (header :: exact ("sec-websocket-version" , "13")) . and (header :: header2 :: < SecWebsocketKey > ()) . and (on_upgrade ()) . map (move | key : SecWebsocketKey , on_upgrade : Option < OnUpgrade > | Ws { config : None , key , on_upgrade , } ,) }
};
}
