// Generated macro for impl_273 (impl)
macro_rules! Depcrate_serdeimpl_273 {
() => {
// Module: crate::serde
// Provides: {"impl_273"}
// Dependencies: {}
# [cfg (feature = "network")] impl Serialize for crate :: NetworkData { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_struct ("NetworkData" , 14) ? ; state . serialize_field ("received" , & self . received ()) ? ; state . serialize_field ("total_received" , & self . total_received ()) ? ; state . serialize_field ("transmitted" , & self . transmitted ()) ? ; state . serialize_field ("total_transmitted" , & self . total_transmitted ()) ? ; state . serialize_field ("packets_received" , & self . packets_received ()) ? ; state . serialize_field ("total_packets_received" , & self . total_packets_received ()) ? ; state . serialize_field ("packets_transmitted" , & self . packets_transmitted ()) ? ; state . serialize_field ("total_packets_transmitted" , & self . total_packets_transmitted () ,) ? ; state . serialize_field ("errors_on_received" , & self . errors_on_received ()) ? ; state . serialize_field ("total_errors_on_received" , & self . total_errors_on_received ()) ? ; state . serialize_field ("errors_on_transmitted" , & self . errors_on_transmitted ()) ? ; state . serialize_field ("total_errors_on_transmitted" , & self . total_errors_on_transmitted () ,) ? ; state . serialize_field ("mac_address" , & self . mac_address ()) ? ; state . serialize_field ("ip_networks" , & self . ip_networks ()) ? ; state . serialize_field ("mtu" , & self . mtu ()) ? ; state . end () } }
};
}
