// Generated macro for GraphQLSubscription (struct)
macro_rules! Depcrate_subscriptionGraphQLSubscription {
() => {
// Module: crate::subscription
// Provides: {"GraphQLSubscription"}
// Dependencies: {}
# [doc = " A GraphQL subscription endpoint builder."] # [cfg_attr (docsrs , doc (cfg (feature = "websocket")))] pub struct GraphQLSubscription < E , OnConnInit , OnPing > { executor : E , on_connection_init : OnConnInit , on_ping : OnPing , keepalive_timeout : Option < Duration > , }
};
}
