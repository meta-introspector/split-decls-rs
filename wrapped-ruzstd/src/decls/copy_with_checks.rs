macro_rules! copy_with_checks {
    () => {
        # [allow (dead_code)] # [inline (always)] # [allow (clippy :: too_many_arguments)] unsafe fn copy_with_checks (m1_ptr : * const u8 , m2_ptr : * const u8 , f1_ptr : * mut u8 , f2_ptr : * mut u8 , m1_in_f1 : usize , m2_in_f1 : usize , m1_in_f2 : usize , m2_in_f2 : usize ,) { if m1_in_f1 != 0 { f1_ptr . copy_from_nonoverlapping (m1_ptr , m1_in_f1) ; } if m2_in_f1 != 0 { f1_ptr . add (m1_in_f1) . copy_from_nonoverlapping (m2_ptr , m2_in_f1) ; } if m1_in_f2 != 0 { f2_ptr . copy_from_nonoverlapping (m1_ptr . add (m1_in_f1) , m1_in_f2) ; } if m2_in_f2 != 0 { f2_ptr . add (m1_in_f2) . copy_from_nonoverlapping (m2_ptr . add (m2_in_f1) , m2_in_f2) ; } }
    };
}

copy_with_checks!();