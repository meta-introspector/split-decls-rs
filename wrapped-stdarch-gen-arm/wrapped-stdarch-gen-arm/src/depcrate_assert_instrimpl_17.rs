// Generated macro for impl_17 (impl)
macro_rules! Depcrate_assert_instrimpl_17 {
() => {
// Module: crate::assert_instr
// Provides: {"impl_17"}
// Dependencies: {}
impl Serialize for InstructionAssertionMethod { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { match self { InstructionAssertionMethod { default : InstructionAssertionMethodForBitsize { default : InstructionAssertion :: Basic (instr) , byte : None , halfword : None , word : None , doubleword : None , } , float : None , unsigned : None , } => serializer . serialize_str (& instr . to_string ()) , InstructionAssertionMethod { default : InstructionAssertionMethodForBitsize { default : InstructionAssertion :: WithArgs (instr , args) , byte : None , halfword : None , word : None , doubleword : None , } , float : None , unsigned : None , } => { let mut seq = serializer . serialize_seq (Some (2)) ? ; seq . serialize_element (& instr . to_string ()) ? ; seq . serialize_element (& args . to_string ()) ? ; seq . end () } _ => InstructionAssertionMethod :: serialize (self , serializer) , } } }
};
}
