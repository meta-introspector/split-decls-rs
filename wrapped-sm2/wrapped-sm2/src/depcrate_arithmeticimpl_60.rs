// Generated macro for impl_60 (impl)
macro_rules! Depcrate_arithmeticimpl_60 {
() => {
// Module: crate::arithmetic
// Provides: {"impl_60"}
// Dependencies: {}
# [doc = " Adapted from [draft-shen-sm2-ecdsa Appendix D]: Recommended Parameters."] # [doc = ""] # [doc = " [draft-shen-sm2-ecdsa Appendix D]: https://datatracker.ietf.org/doc/html/draft-shen-sm2-ecdsa-02#appendix-D"] impl PrimeCurveParams for Sm2 { type FieldElement = FieldElement ; type PointArithmetic = point_arithmetic :: EquationAIsMinusThree ; # [doc = " a = -3 (0xFFFFFFFE FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF 00000000 FFFFFFFF FFFFFFFC)"] const EQUATION_A : FieldElement = FieldElement :: from_u64 (3) . neg () ; # [doc = " b = 0x28E9FA9E 9D9F5E34 4D5A9E4B CF6509A7 F39789F5 15AB8F92 DDBCBD41 4D940E93"] const EQUATION_B : FieldElement = FieldElement :: from_hex_vartime ("28E9FA9E9D9F5E344D5A9E4BCF6509A7F39789F515AB8F92DDBCBD414D940E93" ,) ; # [doc = " Base point of SM2."] # [doc = ""] # [doc = " ```text"] # [doc = " Gₓ = 0x32C4AE2C 1F198119 5F990446 6A39C994 8FE30BBF F2660BE1 715A4589 334C74C7"] # [doc = " Gᵧ = 0xBC3736A2 F4F6779C 59BDCEE3 6B692153 D0A9877C C62A4740 02DF32E5 2139F0A0"] # [doc = " ```"] const GENERATOR : (FieldElement , FieldElement) = (FieldElement :: from_hex_vartime ("32C4AE2C1F1981195F9904466A39C9948FE30BBFF2660BE1715A4589334C74C7" ,) , FieldElement :: from_hex_vartime ("BC3736A2F4F6779C59BDCEE36B692153D0A9877CC62A474002DF32E52139F0A0" ,) ,) ; }
};
}
