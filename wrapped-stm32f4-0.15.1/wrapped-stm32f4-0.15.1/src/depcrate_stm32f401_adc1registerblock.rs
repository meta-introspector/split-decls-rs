// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f401_adc1RegisterBlock {
() => {
// Module: crate::stm32f401::adc1
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , # [doc = "0x04 - control register 1"] pub cr1 : crate :: Reg < cr1 :: CR1_SPEC > , # [doc = "0x08 - control register 2"] pub cr2 : crate :: Reg < cr2 :: CR2_SPEC > , # [doc = "0x0c - sample time register 1"] pub smpr1 : crate :: Reg < smpr1 :: SMPR1_SPEC > , # [doc = "0x10 - sample time register 2"] pub smpr2 : crate :: Reg < smpr2 :: SMPR2_SPEC > , # [doc = "0x14..0x24 - injected channel data offset register x"] pub jofr : [crate :: Reg < jofr :: JOFR_SPEC > ; 4] , # [doc = "0x24 - watchdog higher threshold register"] pub htr : crate :: Reg < htr :: HTR_SPEC > , # [doc = "0x28 - watchdog lower threshold register"] pub ltr : crate :: Reg < ltr :: LTR_SPEC > , # [doc = "0x2c - regular sequence register 1"] pub sqr1 : crate :: Reg < sqr1 :: SQR1_SPEC > , # [doc = "0x30 - regular sequence register 2"] pub sqr2 : crate :: Reg < sqr2 :: SQR2_SPEC > , # [doc = "0x34 - regular sequence register 3"] pub sqr3 : crate :: Reg < sqr3 :: SQR3_SPEC > , # [doc = "0x38 - injected sequence register"] pub jsqr : crate :: Reg < jsqr :: JSQR_SPEC > , # [doc = "0x3c..0x4c - injected data register x"] pub jdr : [crate :: Reg < jdr :: JDR_SPEC > ; 4] , # [doc = "0x4c - regular data register"] pub dr : crate :: Reg < dr :: DR_SPEC > , }
};
}
