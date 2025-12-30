// Generated macro for Sink (trait)
macro_rules! Depcrate_ser_partSink {
() => {
// Module: crate::ser::part
// Provides: {"Sink"}
// Dependencies: {}
pub trait Sink : Sized { type Ok ; fn serialize_static_str (self , value : & 'static str ,) -> Result < Self :: Ok , Error > ; fn serialize_str (self , value : & str) -> Result < Self :: Ok , Error > ; fn serialize_string (self , value : String) -> Result < Self :: Ok , Error > ; fn serialize_none (self) -> Result < Self :: Ok , Error > ; fn serialize_some < T : ? Sized + ser :: Serialize > (self , value : & T ,) -> Result < Self :: Ok , Error > ; fn unsupported (self , type_str : & 'static str) -> Error ; }
};
}
