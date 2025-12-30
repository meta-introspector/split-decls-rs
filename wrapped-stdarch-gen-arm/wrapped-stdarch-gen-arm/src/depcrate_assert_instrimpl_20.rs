// Generated macro for impl_20 (impl)
macro_rules! Depcrate_assert_instrimpl_20 {
() => {
// Module: crate::assert_instr
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a > ToTokens for InstructionAssertionsForBaseType < 'a > { fn to_tokens (& self , tokens : & mut TokenStream) { self . 0 . iter () . for_each (| InstructionAssertionMethod { default , float , unsigned , } | { let kind = self . 1 . map (| ty | ty . kind ()) ; let instruction = match (kind , float , unsigned) { (None , float , unsigned) if float . is_some () || unsigned . is_some () => { unreachable ! ("cannot determine the base type kind for instruction assertion: {self:#?}") } (Some (BaseTypeKind :: Float) , Some (float) , _) => float , (Some (BaseTypeKind :: UInt) , _ , Some (unsigned)) => unsigned , _ => default , } ; let bitsize = self . 1 . and_then (| ty | ty . get_size () . ok ()) ; let instruction = match (bitsize , instruction) { (Some (8) , InstructionAssertionMethodForBitsize { byte : Some (byte) , .. } ,) => byte , (Some (16) , InstructionAssertionMethodForBitsize { halfword : Some (halfword) , .. } ,) => halfword , (Some (32) , InstructionAssertionMethodForBitsize { word : Some (word) , .. } ,) => word , (Some (64) , InstructionAssertionMethodForBitsize { doubleword : Some (doubleword) , .. } ,) => doubleword , (_ , InstructionAssertionMethodForBitsize { default , .. }) => default , } ; tokens . append_all (quote ! { # [cfg_attr (test , assert_instr (# instruction))] }) } ,) ; } }
};
}
