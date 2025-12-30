// Generated macro for CH (struct)
macro_rules! Depcrate_stm32f412_dfsdmCH {
() => {
// Module: crate::stm32f412::dfsdm
// Provides: {"CH"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct CH { # [doc = "0x00 - channel configuration y register"] pub cfgr1 : crate :: Reg < self :: ch :: cfgr1 :: CFGR1_SPEC > , # [doc = "0x04 - channel configuration y register"] pub cfgr2 : crate :: Reg < self :: ch :: cfgr2 :: CFGR2_SPEC > , # [doc = "0x08 - analog watchdog and short-circuit detector register"] pub awscdr : crate :: Reg < self :: ch :: awscdr :: AWSCDR_SPEC > , # [doc = "0x0c - channel watchdog filter data register"] pub wdatr : crate :: Reg < self :: ch :: wdatr :: WDATR_SPEC > , # [doc = "0x10 - channel data input register"] pub datinr : crate :: Reg < self :: ch :: datinr :: DATINR_SPEC > , }
};
}
