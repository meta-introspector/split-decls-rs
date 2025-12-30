// Generated macro for impl_35 (impl)
macro_rules! Depcrate_hypervisorimpl_35 {
() => {
// Module: crate::hypervisor
// Provides: {"impl_35"}
// Dependencies: {}
impl PhysicalMemory { # [doc = " Allocate a chunk of memory that is handed out as \"physical memory\""] # [doc = " to allocate page-tables etc."] pub (crate) fn new (offset : u64) -> PhysicalMemory { let size = 4 * (1 << 20) ; let options = [MapOption :: MapAddr (offset as * const u8) , MapOption :: MapReadable , MapOption :: MapWritable , MapOption :: MapExecutable ,] ; let mm = MemoryMap :: new (size , & options) . unwrap () ; PhysicalMemory { offset : offset as usize , allocated : 0 , size : size , backing_memory : mm , } } fn len (& self) -> usize { self . size } fn alloc_pages (& mut self , how_many : u64) -> * mut u8 { let to_allocate = how_many as usize * BASE_PAGE_SIZE ; if self . allocated + to_allocate > self . size { panic ! ("OOM") } let ptr = (self . offset + self . allocated) as * mut u8 ; self . allocated += to_allocate ; ptr } }
};
}
