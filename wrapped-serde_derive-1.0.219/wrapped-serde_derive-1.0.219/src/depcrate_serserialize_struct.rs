// Generated macro for serialize_struct (function)
macro_rules! Depcrate_serserialize_struct {
() => {
// Module: crate::ser
// Provides: {"serialize_struct"}
// Dependencies: {}
fn serialize_struct (params : & Parameters , fields : & [Field] , cattrs : & attr :: Container) -> Fragment { assert ! (fields . len () as u64 <= u64 :: from (u32 :: MAX) , "too many fields in {}: {}, maximum supported count is {}" , cattrs . name () . serialize_name () , fields . len () , u32 :: MAX ,) ; let has_non_skipped_flatten = fields . iter () . any (| field | field . attrs . flatten () && ! field . attrs . skip_serializing ()) ; if has_non_skipped_flatten { serialize_struct_as_map (params , fields , cattrs) } else { serialize_struct_as_struct (params , fields , cattrs) } }
};
}
