// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f446_gpiohRegisterBlock {
() => {
// Module: crate::stm32f446::gpioh
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - GPIO port mode register"] pub moder : crate :: Reg < moder :: MODER_SPEC > , # [doc = "0x04 - GPIO port output type register"] pub otyper : crate :: Reg < otyper :: OTYPER_SPEC > , # [doc = "0x08 - GPIO port output speed register"] pub ospeedr : crate :: Reg < ospeedr :: OSPEEDR_SPEC > , # [doc = "0x0c - GPIO port pull-up/pull-down register"] pub pupdr : crate :: Reg < pupdr :: PUPDR_SPEC > , # [doc = "0x10 - GPIO port input data register"] pub idr : crate :: Reg < idr :: IDR_SPEC > , # [doc = "0x14 - GPIO port output data register"] pub odr : crate :: Reg < odr :: ODR_SPEC > , # [doc = "0x18 - GPIO port bit set/reset register"] pub bsrr : crate :: Reg < bsrr :: BSRR_SPEC > , # [doc = "0x1c - GPIO port configuration lock register"] pub lckr : crate :: Reg < lckr :: LCKR_SPEC > , # [doc = "0x20 - GPIO alternate function low register"] pub afrl : crate :: Reg < afrl :: AFRL_SPEC > , # [doc = "0x24 - GPIO alternate function high register"] pub afrh : crate :: Reg < afrh :: AFRH_SPEC > , }
};
}
