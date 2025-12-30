// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f469_usart1RegisterBlock {
() => {
// Module: crate::stm32f469::usart1
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - Status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , # [doc = "0x04 - Data register"] pub dr : crate :: Reg < dr :: DR_SPEC > , # [doc = "0x08 - Baud rate register"] pub brr : crate :: Reg < brr :: BRR_SPEC > , # [doc = "0x0c - Control register 1"] pub cr1 : crate :: Reg < cr1 :: CR1_SPEC > , # [doc = "0x10 - Control register 2"] pub cr2 : crate :: Reg < cr2 :: CR2_SPEC > , # [doc = "0x14 - Control register 3"] pub cr3 : crate :: Reg < cr3 :: CR3_SPEC > , # [doc = "0x18 - Guard time and prescaler register"] pub gtpr : crate :: Reg < gtpr :: GTPR_SPEC > , }
};
}
