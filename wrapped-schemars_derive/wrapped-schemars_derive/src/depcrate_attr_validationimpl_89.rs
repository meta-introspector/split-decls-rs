// Generated macro for impl_89 (impl)
macro_rules! Depcrate_attr_validationimpl_89 {
() => {
// Module: crate::attr::validation
// Provides: {"impl_89"}
// Dependencies: {}
impl Format { fn attr_str (self) -> & 'static str { match self { Format :: Email => "email" , Format :: Uri => "url" , Format :: Ip => "ip" , Format :: Ipv4 => "ipv4" , Format :: Ipv6 => "ipv6" , } } fn schema_str (self) -> & 'static str { match self { Format :: Email => "email" , Format :: Uri => "uri" , Format :: Ip => "ip" , Format :: Ipv4 => "ipv4" , Format :: Ipv6 => "ipv6" , } } fn from_attr_str (s : & str) -> Option < Self > { Some (match s { "email" => Format :: Email , "url" => Format :: Uri , "ip" => Format :: Ip , "ipv4" => Format :: Ipv4 , "ipv6" => Format :: Ipv6 , _ => return None , }) } }
};
}
