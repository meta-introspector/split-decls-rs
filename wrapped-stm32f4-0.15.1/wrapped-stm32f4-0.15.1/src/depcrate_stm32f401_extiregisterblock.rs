// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f401_extiRegisterBlock {
() => {
// Module: crate::stm32f401::exti
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - Interrupt mask register (EXTI_IMR)"] pub imr : crate :: Reg < imr :: IMR_SPEC > , # [doc = "0x04 - Event mask register (EXTI_EMR)"] pub emr : crate :: Reg < emr :: EMR_SPEC > , # [doc = "0x08 - Rising Trigger selection register (EXTI_RTSR)"] pub rtsr : crate :: Reg < rtsr :: RTSR_SPEC > , # [doc = "0x0c - Falling Trigger selection register (EXTI_FTSR)"] pub ftsr : crate :: Reg < ftsr :: FTSR_SPEC > , # [doc = "0x10 - Software interrupt event register (EXTI_SWIER)"] pub swier : crate :: Reg < swier :: SWIER_SPEC > , # [doc = "0x14 - Pending register (EXTI_PR)"] pub pr : crate :: Reg < pr :: PR_SPEC > , }
};
}
