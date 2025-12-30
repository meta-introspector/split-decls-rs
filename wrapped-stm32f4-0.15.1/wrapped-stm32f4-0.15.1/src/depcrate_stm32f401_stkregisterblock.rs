// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f401_stkRegisterBlock {
() => {
// Module: crate::stm32f401::stk
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - SysTick control and status register"] pub ctrl : crate :: Reg < ctrl :: CTRL_SPEC > , # [doc = "0x04 - SysTick reload value register"] pub load : crate :: Reg < load :: LOAD_SPEC > , # [doc = "0x08 - SysTick current value register"] pub val : crate :: Reg < val :: VAL_SPEC > , # [doc = "0x0c - SysTick calibration value register"] pub calib : crate :: Reg < calib :: CALIB_SPEC > , }
};
}
