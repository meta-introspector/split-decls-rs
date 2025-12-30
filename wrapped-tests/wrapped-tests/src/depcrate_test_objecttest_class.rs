// Generated macro for test_class (function)
macro_rules! Depcrate_test_objecttest_class {
() => {
// Module: crate::test_object
// Provides: {"test_class"}
// Dependencies: {}
# [test] # [cfg_attr (all (target_vendor = "apple" , not (target_arch = "aarch64")) , ignore = "has trouble linking")] fn test_class () { let cls = MyTestObject :: class () ; assert_eq ! (MyTestObject :: add_numbers (- 3 , 15) , 12) ; let classes = AnyClass :: classes () ; assert_eq ! (classes . len () , AnyClass :: classes_count ()) ; assert_in ! (cls , classes) ; assert_eq ! (AnyClass :: get (& c ("MyTestObject")) , Some (cls)) ; assert_ne ! (cls , class ! (NSObject)) ; assert_eq ! (cls . name () , &* c ("MyTestObject")) ; assert_eq ! (cls . superclass () , Some (class ! (NSObject))) ; assert_eq ! (cls . metaclass () . name () , &* c ("MyTestObject")) ; assert_ne ! (cls . metaclass () , cls) ; assert_eq ! (cls . instance_size () , { # [repr (C)] struct MyTestObjectLayout { isa : * const AnyClass , var1 : c_int , var2 : Bool , var3 : * mut NSObject , } size_of ::< MyTestObjectLayout > () }) ; let protocol = AnyProtocol :: get (& c ("NSObject")) . unwrap () ; assert ! (cls . conforms_to (protocol)) ; assert ! (! cls . conforms_to (AnyProtocol :: get (& c ("NSCopying")) . unwrap ())) ; assert_not_in ! (protocol , cls . adopted_protocols ()) ; assert_in ! (AnyProtocol :: get (& c ("MyTestProtocol")) . unwrap () , cls . adopted_protocols ()) ; let method = cls . instance_method (sel ! (addToVar1 :)) . unwrap () ; assert_in ! (method , cls . instance_methods ()) ; let ivar = cls . instance_variable (& c ("var1")) . unwrap () ; assert_in ! (ivar , cls . instance_variables ()) ; }
};
}
