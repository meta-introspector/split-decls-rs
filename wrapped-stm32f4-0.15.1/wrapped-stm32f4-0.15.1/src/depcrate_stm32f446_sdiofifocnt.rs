// Generated macro for fifocnt (module)
macro_rules! Depcrate_stm32f446_sdiofifocnt {
() => {
// Module: crate::stm32f446::sdio
// Provides: {"fifocnt"}
// Dependencies: {}
# [doc = "The SDIO_FIFOCNT register contains the remaining number of words to be written to or read from the FIFO. The FIFO counter loads the value from the data length register (see SDIO_DLEN) when the data transfer enable bit, DTEN, is set in the data control register (SDIO_DCTRL register) and the DPSM is at the Idle state. If the data length is not word-aligned (multiple of 4), the remaining 1 to 3 bytes are regarded as a word."] pub mod fifocnt ;
};
}
