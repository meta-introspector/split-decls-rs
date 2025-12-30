// Generated macro for impl_84 (impl)
macro_rules! Depcrate_dependenciesimpl_84 {
() => {
// Module: crate::dependencies
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Dependency { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct DependencyVisitor ; impl < 'de > Visitor < 'de > for DependencyVisitor { type Value = Dependency ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a version string like \"0.9.8\" or a \
                     dependency like { version = \"0.9.8\" }" ,) } fn visit_str < E > (self , s : & str) -> Result < Self :: Value , E > where E : de :: Error , { Ok (Dependency { version : Some (s . to_owned ()) , path : None , optional : false , default_features : Some (true) , features : Vec :: new () , git : None , branch : None , tag : None , rev : None , workspace : false , rest : Map :: new () , }) } fn visit_map < M > (self , map : M) -> Result < Self :: Value , M :: Error > where M : de :: MapAccess < 'de > , { Dependency :: deserialize (MapAccessDeserializer :: new (map)) } } deserializer . deserialize_any (DependencyVisitor) } }
};
}
