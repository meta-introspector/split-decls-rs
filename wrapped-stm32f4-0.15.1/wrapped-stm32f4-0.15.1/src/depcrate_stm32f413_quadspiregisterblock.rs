// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f413_quadspiRegisterBlock {
() => {
// Module: crate::stm32f413::quadspi
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - control register"] pub cr : crate :: Reg < cr :: CR_SPEC > , # [doc = "0x04 - device configuration register"] pub dcr : crate :: Reg < dcr :: DCR_SPEC > , # [doc = "0x08 - status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , # [doc = "0x0c - flag clear register"] pub fcr : crate :: Reg < fcr :: FCR_SPEC > , # [doc = "0x10 - data length register"] pub dlr : crate :: Reg < dlr :: DLR_SPEC > , # [doc = "0x14 - communication configuration register"] pub ccr : crate :: Reg < ccr :: CCR_SPEC > , # [doc = "0x18 - address register"] pub ar : crate :: Reg < ar :: AR_SPEC > , # [doc = "0x1c - ABR"] pub abr : crate :: Reg < abr :: ABR_SPEC > , # [doc = "0x20 - data register"] pub dr : crate :: Reg < dr :: DR_SPEC > , # [doc = "0x24 - polling status mask register"] pub psmkr : crate :: Reg < psmkr :: PSMKR_SPEC > , # [doc = "0x28 - polling status match register"] pub psmar : crate :: Reg < psmar :: PSMAR_SPEC > , # [doc = "0x2c - polling interval register"] pub pir : crate :: Reg < pir :: PIR_SPEC > , # [doc = "0x30 - low-power timeout register"] pub lptr : crate :: Reg < lptr :: LPTR_SPEC > , }
};
}
