// Generated macro for impl_250 (impl)
macro_rules! Depcrate_debugimpl_250 {
() => {
// Module: crate::debug
// Provides: {"impl_250"}
// Dependencies: {}
# [cfg (feature = "network")] impl std :: fmt :: Debug for crate :: NetworkData { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("NetworkData") . field ("income" , & self . received ()) . field ("total income" , & self . total_received ()) . field ("outcome" , & self . transmitted ()) . field ("total outcome" , & self . total_transmitted ()) . field ("packets income" , & self . packets_received ()) . field ("total packets income" , & self . total_packets_received ()) . field ("packets outcome" , & self . packets_transmitted ()) . field ("total packets outcome" , & self . total_packets_transmitted ()) . field ("errors income" , & self . errors_on_received ()) . field ("total errors income" , & self . total_errors_on_received ()) . field ("errors outcome" , & self . errors_on_transmitted ()) . field ("total errors outcome" , & self . total_errors_on_transmitted ()) . field ("maximum transfer unit" , & self . mtu ()) . finish () } }
};
}
