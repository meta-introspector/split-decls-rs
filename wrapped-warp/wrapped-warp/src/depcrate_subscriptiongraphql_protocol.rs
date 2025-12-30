// Generated macro for graphql_protocol (function)
macro_rules! Depcrate_subscriptiongraphql_protocol {
() => {
// Module: crate::subscription
// Provides: {"graphql_protocol"}
// Dependencies: {}
# [doc = " Create a `Filter` that parse [WebSocketProtocols] from"] # [doc = " `sec-websocket-protocol` header."] pub fn graphql_protocol () -> impl Filter < Extract = (WebSocketProtocols ,) , Error = Rejection > + Clone { warp :: header :: optional :: < String > ("sec-websocket-protocol") . map (| protocols : Option < String > | { protocols . and_then (| protocols | { protocols . split (',') . find_map (| p | WebSocketProtocols :: from_str (p . trim ()) . ok ()) }) . unwrap_or (WebSocketProtocols :: SubscriptionsTransportWS) }) }
};
}
