// Generated macro for impl_15640 (impl)
macro_rules! Depcrate_stm32f446_sdioimpl_15640 {
() => {
// Module: crate::stm32f446::sdio
// Provides: {"impl_15640"}
// Dependencies: {}
impl RegisterBlock { # [doc = "0x14 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."] # [inline (always)] pub fn resp1 (& self) -> & crate :: Reg < resp :: RESP_SPEC > { & self . resp [0] } # [doc = "0x18 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."] # [inline (always)] pub fn resp2 (& self) -> & crate :: Reg < resp :: RESP_SPEC > { & self . resp [1] } # [doc = "0x1c - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."] # [inline (always)] pub fn resp3 (& self) -> & crate :: Reg < resp :: RESP_SPEC > { & self . resp [2] } # [doc = "0x20 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response."] # [inline (always)] pub fn resp4 (& self) -> & crate :: Reg < resp :: RESP_SPEC > { & self . resp [3] } }
};
}
