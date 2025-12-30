// Generated macro for mem_free (function)
macro_rules! Depcrate_inflatemem_free {
() => {
// Module: crate::inflate
// Provides: {"mem_free"}
// Dependencies: {}
unsafe extern "C" fn mem_free (mem : * mut c_void , ptr : * mut c_void) { extern "C" { fn free (p : * mut c_void) ; } if mem . is_null () { unsafe { free (ptr) } ; return ; } let mut zone = ManuallyDrop :: new (unsafe { Box :: from_raw (mem as * mut MemZone) }) ; let last = zone . items . pop () ; let mut found = None ; if let Some (last) = last { if last . ptr == ptr { found = Some (last) ; } else { zone . items . push (last) ; let index = zone . items . iter () . position (| item | item . ptr == ptr) ; if let Some (index) = index { let last = zone . items . remove (index) ; found = Some (last) ; zone . not_lifo += 1 ; } } } if let Some (item) = found { zone . total -= item . size ; } else { zone . rogue += 1 ; } unsafe { free (ptr) } }
};
}
