// Generated macro for test_object (function)
macro_rules! Depcrate_test_objecttest_object {
() => {
// Module: crate::test_object
// Provides: {"test_object"}
// Dependencies: {}
# [test] # [cfg_attr (all (target_vendor = "apple" , not (target_arch = "aarch64")) , ignore = "has trouble linking")] fn test_object () { autoreleasepool (| pool | { let _obj = unsafe { MyTestObject :: new_autoreleased (pool) } ; }) ; let _obj = MyTestObject :: new_autoreleased_retained () ; let obj = MyTestObject :: new () ; assert_eq ! ((** obj) . class () , MyTestObject :: class ()) ; assert_eq ! (obj . var1 () , 42) ; assert_eq ! (* obj . var1_ivar () , 42) ; obj . add_to_ivar1 (3) ; assert_eq ! (obj . var1 () , 45) ; assert_eq ! (* obj . var1_ivar () , 45) ; unsafe { * obj . var1_ivar_ptr () = 100 } ; assert_eq ! (obj . var1 () , 100) ; assert_eq ! (* obj . var1_ivar () , 100) ; assert ! (obj . var2 ()) ; assert ! (obj . var2_ivar () . is_true ()) ; unsafe { * obj . var2_ivar_ptr () = Bool :: NO } ; assert ! (! obj . var2 ()) ; assert ! (obj . var2_ivar () . is_false ()) ; assert ! (obj . var3 () . is_null ()) ; assert ! (obj . var3_ivar () . is_null ()) ; let obj2 = Retained :: as_ptr (& * ManuallyDrop :: new (NSObject :: new ())) as _ ; obj . set_var3 (obj2) ; assert_eq ! (obj . var3 () , obj2) ; assert_eq ! (* obj . var3_ivar () , obj2) ; let obj3 = Retained :: as_ptr (& * ManuallyDrop :: new (NSObject :: new ())) as _ ; unsafe { * obj . var3_ivar_ptr () = obj3 } ; assert_ne ! (obj . var3 () , obj2) ; assert_ne ! (* obj . var3_ivar () , obj2) ; assert_eq ! (obj . var3 () , obj3) ; assert_eq ! (* obj . var3_ivar () , obj3) ; }
};
}
