// Generated macro for SerializedEvent (struct)
macro_rules! DepcrateSerializedEvent {
() => {
// Module: crate
// Provides: {"SerializedEvent"}
// Dependencies: {}
# [derive (Serialize)] struct SerializedEvent < 'a > { # [serde (flatten)] event : SerializeEventFieldMapStrippingLog < 'a > , # [serde (flatten)] extra_fields : & 'a HashMap < String , String > , # [serde (flatten)] span_fields : serde_json :: Map < String , serde_json :: Value > , _spans : & 'a [& 'a str] , _target : & 'a str , _module_path : Option < & 'a str > , _file : Option < & 'a str > , _line : Option < u32 > , }
};
}
