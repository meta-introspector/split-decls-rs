// Generated macro for impl_45 (impl)
macro_rules! Depcrate_deimpl_45 {
() => {
// Module: crate::de
// Provides: {"impl_45"}
// Dependencies: {}
impl < 'de > Deserializer < 'de > { pub fn new (tokens : & 'de [Token]) -> Self { Deserializer { tokens } } fn peek_token_opt (& self) -> Option < Token > { self . tokens . first () . copied () } fn peek_token (& self) -> Result < Token , Error > { self . peek_token_opt () . ok_or_else (end_of_tokens) } pub fn next_token_opt (& mut self) -> Option < Token > { match self . tokens . split_first () { Some ((& first , rest)) => { self . tokens = rest ; Some (first) } None => None , } } fn next_token (& mut self) -> Result < Token , Error > { let (& first , rest) = self . tokens . split_first () . ok_or_else (end_of_tokens) ? ; self . tokens = rest ; Ok (first) } pub fn remaining (& self) -> usize { self . tokens . len () } fn visit_seq < V > (& mut self , len : Option < usize > , end : Token , visitor : V ,) -> Result < V :: Value , Error > where V : Visitor < 'de > , { let value = visitor . visit_seq (DeserializerSeqVisitor { de : self , len , end }) ? ; assert_next_token (self , end) ? ; Ok (value) } fn visit_map < V > (& mut self , len : Option < usize > , end : Token , visitor : V ,) -> Result < V :: Value , Error > where V : Visitor < 'de > , { let value = visitor . visit_map (DeserializerMapVisitor { de : self , len , end }) ? ; assert_next_token (self , end) ? ; Ok (value) } }
};
}
