// Generated macro for impl_322 (impl)
macro_rules! Depcrate_http3_settingsimpl_322 {
() => {
// Module: crate::http3::settings
// Provides: {"impl_322"}
// Dependencies: {}
impl From < & Http3Settings > for quiche :: h3 :: Config { fn from (value : & Http3Settings) -> Self { let mut config = Self :: new () . unwrap () ; if let Some (v) = value . max_header_list_size { config . set_max_field_section_size (v) ; } if let Some (v) = value . qpack_max_table_capacity { config . set_qpack_max_table_capacity (v) ; } if let Some (v) = value . qpack_blocked_streams { config . set_qpack_blocked_streams (v) ; } if value . enable_extended_connect { config . enable_extended_connect (value . enable_extended_connect) } config } }
};
}
