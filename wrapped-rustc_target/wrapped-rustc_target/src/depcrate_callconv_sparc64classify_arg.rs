// Generated macro for classify_arg (function)
macro_rules! Depcrate_callconv_sparc64classify_arg {
() => {
// Module: crate::callconv::sparc64
// Provides: {"classify_arg"}
// Dependencies: {}
fn classify_arg < 'a , Ty , C > (cx : & C , arg : & mut ArgAbi < 'a , Ty > , in_registers_max : Size) where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { if ! arg . layout . is_aggregate () { arg . extend_integer_width_to (64) ; return ; } let total = arg . layout . size ; if total > in_registers_max { arg . make_indirect () ; return ; } match arg . layout . fields { FieldsShape :: Primitive => unreachable ! () , FieldsShape :: Array { .. } => { arg . make_indirect () ; return ; } FieldsShape :: Union (_) => { } FieldsShape :: Arbitrary { .. } => { let mut data = parse_structure (cx , arg . layout , Sdata { prefix : [None ; 8] , prefix_index : 0 , last_offset : Size :: ZERO , has_float : false , arg_attribute : ArgAttribute :: default () , } , Size :: ZERO ,) ; if data . has_float { if data . last_offset < arg . layout . size && ! data . last_offset . bytes () . is_multiple_of (8) && data . prefix_index < data . prefix . len () { data . prefix [data . prefix_index] = Some (Reg :: i32 ()) ; data . prefix_index += 1 ; data . last_offset += Reg :: i32 () . size ; } let mut rest_size = arg . layout . size - data . last_offset ; if ! rest_size . bytes () . is_multiple_of (8) && data . prefix_index < data . prefix . len () { data . prefix [data . prefix_index] = Some (Reg :: i32 ()) ; rest_size = rest_size - Reg :: i32 () . size ; } arg . cast_to (CastTarget :: prefixed (data . prefix , Uniform :: new (Reg :: i64 () , rest_size)) . with_attrs (data . arg_attribute . into ()) ,) ; return ; } } } arg . cast_to (Uniform :: new (Reg :: i64 () , total)) ; }
};
}
