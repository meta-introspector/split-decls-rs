// Generated macro for EventDescCreate (function)
macro_rules! Depcrate_shared_evntprovEventDescCreate {
() => {
// Module: crate::shared::evntprov
// Provides: {"EventDescCreate"}
// Dependencies: {}
# [inline] pub unsafe fn EventDescCreate (EventDescriptor : PEVENT_DESCRIPTOR , Id : USHORT , Version : UCHAR , Channel : UCHAR , Level : UCHAR , Task : USHORT , Opcode : UCHAR , Keyword : ULONGLONG ,) { (* EventDescriptor) . Id = Id ; (* EventDescriptor) . Version = Version ; (* EventDescriptor) . Channel = Channel ; (* EventDescriptor) . Level = Level ; (* EventDescriptor) . Task = Task ; (* EventDescriptor) . Opcode = Opcode ; (* EventDescriptor) . Keyword = Keyword ; }
};
}
