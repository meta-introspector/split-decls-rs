// Generated macro for impl_17 (impl)
macro_rules! Depcrate_zlintimpl_17 {
() => {
// Module: crate::zlint
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for LintStatus { fn deserialize < D > (deserializer : D) -> Result < LintStatus , D :: Error > where D : Deserializer < 'de > , { struct StatusVisitor ; impl < 'de > Visitor < 'de > for StatusVisitor { type Value = LintStatus ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("an integer between -2^31 and 2^31") } fn visit_map < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : MapAccess < 'de > , { let mut status_output = None ; let mut details = None ; while let Some ((key , value)) = access . next_entry :: < & str , & str > () ? { if key == "result" { status_output = Some (match value { "NA" => Status :: NotApplicable , "NE" => Status :: NotEffective , "pass" => Status :: Pass , "notice" => Status :: Notice , "fatal" => Status :: Fatal , "error" => Status :: Error , "warn" => Status :: Warn , "info" => Status :: Info , other => { return Err (M :: Error :: custom (format ! ("unsupported value: {other}" ,) ,)) } }) ; } if key == "details" { details = Some (value . to_string ()) ; } } if let Some (status) = status_output { Ok (LintStatus { status , details }) } else { Err (M :: Error :: custom ("no 'result' field found")) } } } deserializer . deserialize_map (StatusVisitor) } }
};
}
