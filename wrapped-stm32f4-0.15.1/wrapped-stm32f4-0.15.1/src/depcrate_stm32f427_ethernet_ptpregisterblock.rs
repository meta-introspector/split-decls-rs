// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f427_ethernet_ptpRegisterBlock {
() => {
// Module: crate::stm32f427::ethernet_ptp
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - Ethernet PTP time stamp control register"] pub ptptscr : crate :: Reg < ptptscr :: PTPTSCR_SPEC > , # [doc = "0x04 - Ethernet PTP subsecond increment register"] pub ptpssir : crate :: Reg < ptpssir :: PTPSSIR_SPEC > , # [doc = "0x08 - Ethernet PTP time stamp high register"] pub ptptshr : crate :: Reg < ptptshr :: PTPTSHR_SPEC > , # [doc = "0x0c - Ethernet PTP time stamp low register"] pub ptptslr : crate :: Reg < ptptslr :: PTPTSLR_SPEC > , # [doc = "0x10 - Ethernet PTP time stamp high update register"] pub ptptshur : crate :: Reg < ptptshur :: PTPTSHUR_SPEC > , # [doc = "0x14 - Ethernet PTP time stamp low update register"] pub ptptslur : crate :: Reg < ptptslur :: PTPTSLUR_SPEC > , # [doc = "0x18 - Ethernet PTP time stamp addend register"] pub ptptsar : crate :: Reg < ptptsar :: PTPTSAR_SPEC > , # [doc = "0x1c - Ethernet PTP target time high register"] pub ptptthr : crate :: Reg < ptptthr :: PTPTTHR_SPEC > , # [doc = "0x20 - Ethernet PTP target time low register"] pub ptpttlr : crate :: Reg < ptpttlr :: PTPTTLR_SPEC > , _reserved9 : [u8 ; 0x04] , # [doc = "0x28 - Ethernet PTP time stamp status register"] pub ptptssr : crate :: Reg < ptptssr :: PTPTSSR_SPEC > , # [doc = "0x2c - Ethernet PTP PPS control register"] pub ptpppscr : crate :: Reg < ptpppscr :: PTPPPSCR_SPEC > , }
};
}
