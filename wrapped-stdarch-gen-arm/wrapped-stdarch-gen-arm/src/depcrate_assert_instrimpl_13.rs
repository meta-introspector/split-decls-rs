// Generated macro for impl_13 (impl)
macro_rules! Depcrate_assert_instrimpl_13 {
() => {
// Module: crate::assert_instr
// Provides: {"impl_13"}
// Dependencies: {}
impl Serialize for InstructionAssertionMethodForBitsize { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { match self { InstructionAssertionMethodForBitsize { default : InstructionAssertion :: Basic (instr) , byte : None , halfword : None , word : None , doubleword : None , } => serializer . serialize_str (& instr . to_string ()) , InstructionAssertionMethodForBitsize { default : InstructionAssertion :: WithArgs (instr , args) , byte : None , halfword : None , word : None , doubleword : None , } => { let mut seq = serializer . serialize_seq (Some (2)) ? ; seq . serialize_element (& instr . to_string ()) ? ; seq . serialize_element (& args . to_string ()) ? ; seq . end () } _ => InstructionAssertionMethodForBitsize :: serialize (self , serializer) , } } }
};
}
