// Generated macro for impl_59 (impl)
macro_rules! Depcrate_bagimpl_59 {
() => {
// Module: crate::bag
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'b , T , const ARRAY_LEN : usize > Iterator for IterMut < 'b , T , ARRAY_LEN > { type Item = & 'b mut T ; # [inline] fn next (& mut self) -> Option < Self :: Item > { while self . current_index != u32 :: MAX { let current_storage = if let Some (linked) = self . current_stack_entry . as_mut () { & mut * * linked } else { & mut self . bag . primary_storage } ; let instance_bitmap = Storage :: < T , ARRAY_LEN > :: instance_bitmap (current_storage . metadata . load (Acquire)) ; let first_occupied = (instance_bitmap . wrapping_shr (self . current_index)) . trailing_zeros () ; let next_occupied = self . current_index + first_occupied ; self . current_index = next_occupied + 1 ; if (next_occupied as usize) < ARRAY_LEN { return Some (unsafe { & mut * (* current_storage . storage . get ()) [next_occupied as usize] . as_mut_ptr () }) ; } self . current_index = u32 :: MAX ; if let Some (linked) = self . current_stack_entry . as_mut () { let guard = Guard :: new () ; if let Some (next) = linked . next_ptr (Acquire , & guard) . as_ref () { let entry_mut = ptr :: from_ref (next) . cast_mut () ; self . current_stack_entry = unsafe { entry_mut . as_mut () } ; self . current_index = 0 ; } } else { self . bag . stack . peek_with (| e | { if let Some (e) = e { let entry_mut = ptr :: from_ref (e) . cast_mut () ; self . current_stack_entry = unsafe { entry_mut . as_mut () } ; self . current_index = 0 ; } }) ; } } None } }
};
}
