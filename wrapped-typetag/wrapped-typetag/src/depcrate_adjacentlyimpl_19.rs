// Generated macro for impl_19 (impl)
macro_rules! Depcrate_adjacentlyimpl_19 {
() => {
// Module: crate::adjacently
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'de > Visitor < 'de > for TagContentOtherFieldVisitor { type Value = TagContentOtherField ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { let [tag_field_name , content_field_name] = * self . field_names ; if self . deny_unknown_fields { write ! (formatter , "{:?} or {:?}" , tag_field_name , content_field_name ,) } else { write ! (formatter , "{:?}, {:?}, or other ignored fields" , tag_field_name , content_field_name ,) } } fn visit_str < E > (self , field : & str) -> Result < Self :: Value , E > where E : de :: Error , { let [tag_field_name , content_field_name] = * self . field_names ; if field == tag_field_name { Ok (TagContentOtherField :: Tag) } else if field == content_field_name { Ok (TagContentOtherField :: Content) } else if self . deny_unknown_fields { Err (E :: unknown_field (field , self . field_names)) } else { Ok (TagContentOtherField :: Other) } } }
};
}
