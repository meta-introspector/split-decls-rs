// Generated macro for GetEventProcessorIndex (function)
macro_rules! Depcrate_um_evntconsGetEventProcessorIndex {
() => {
// Module: crate::um::evntcons
// Provides: {"GetEventProcessorIndex"}
// Dependencies: {}
# [inline] pub unsafe fn GetEventProcessorIndex (EventRecord : PCEVENT_RECORD) -> ULONG { if (* EventRecord) . EventHeader . Flags & EVENT_HEADER_FLAG_PROCESSOR_INDEX != 0 { * (* EventRecord) . BufferContext . u . ProcessorIndex () as ULONG } else { (* EventRecord) . BufferContext . u . s () . ProcessorNumber as ULONG } }
};
}
