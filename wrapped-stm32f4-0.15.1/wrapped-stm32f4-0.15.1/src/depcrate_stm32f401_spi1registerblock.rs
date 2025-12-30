// Generated macro for RegisterBlock (struct)
macro_rules! Depcrate_stm32f401_spi1RegisterBlock {
() => {
// Module: crate::stm32f401::spi1
// Provides: {"RegisterBlock"}
// Dependencies: {}
# [doc = r"Register block"] # [repr (C)] pub struct RegisterBlock { # [doc = "0x00 - control register 1"] pub cr1 : crate :: Reg < cr1 :: CR1_SPEC > , # [doc = "0x04 - control register 2"] pub cr2 : crate :: Reg < cr2 :: CR2_SPEC > , # [doc = "0x08 - status register"] pub sr : crate :: Reg < sr :: SR_SPEC > , # [doc = "0x0c - data register"] pub dr : crate :: Reg < dr :: DR_SPEC > , # [doc = "0x10 - CRC polynomial register"] pub crcpr : crate :: Reg < crcpr :: CRCPR_SPEC > , # [doc = "0x14 - RX CRC register"] pub rxcrcr : crate :: Reg < rxcrcr :: RXCRCR_SPEC > , # [doc = "0x18 - TX CRC register"] pub txcrcr : crate :: Reg < txcrcr :: TXCRCR_SPEC > , # [doc = "0x1c - I2S configuration register"] pub i2scfgr : crate :: Reg < i2scfgr :: I2SCFGR_SPEC > , # [doc = "0x20 - I2S prescaler register"] pub i2spr : crate :: Reg < i2spr :: I2SPR_SPEC > , }
};
}
