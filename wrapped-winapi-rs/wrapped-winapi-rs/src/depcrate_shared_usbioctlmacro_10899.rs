// Generated macro for macro_10899 (macro)
macro_rules! Depcrate_shared_usbioctlmacro_10899 {
() => {
// Module: crate::shared::usbioctl
// Provides: {"macro_10899"}
// Dependencies: {}
STRUCT ! { # [repr (packed)] struct USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION { TimeTrackingHandle : HANDLE , InputFrameNumber : ULONG , InputMicroFrameNumber : ULONG , QueryPerformanceCounterAtInputFrameOrMicroFrame : LARGE_INTEGER , QueryPerformanceCounterFrequency : LARGE_INTEGER , PredictedAccuracyInMicroSeconds : ULONG , CurrentGenerationID : ULONG , CurrentQueryPerformanceCounter : LARGE_INTEGER , CurrentHardwareFrameNumber : ULONG , CurrentHardwareMicroFrameNumber : ULONG , CurrentUSBFrameNumber : ULONG , } }
};
}
