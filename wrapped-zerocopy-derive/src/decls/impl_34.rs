macro_rules! deps {
    () => {
        Repr!();
        PrimitiveRepr!();
        AlignRepr!();
        CompoundRepr!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < Prim , Packed > Repr < Prim , Packed > { # [doc = " Gets the name of this \"repr type\" - the non-align `repr(X)` that is used"] # [doc = " in prose to refer to this type."] # [doc = ""] # [doc = " For example, we would refer to `#[repr(C, align(4))] struct Foo { ... }`"] # [doc = " as a \"`repr(C)` struct\"."] pub (crate) fn repr_type_name (& self) -> & str where Prim : Copy + With < PrimitiveRepr > , { use CompoundRepr :: * ; use PrimitiveRepr :: * ; use Repr :: * ; match self { Transparent (_span) => "repr(transparent)" , Compound (Spanned { t : repr , span : _ } , _align) => match repr { C => "repr(C)" , Rust => "repr(Rust)" , Primitive (prim) => prim . with (| prim | match prim { U8 => "repr(u8)" , U16 => "repr(u16)" , U32 => "repr(u32)" , U64 => "repr(u64)" , U128 => "repr(u128)" , Usize => "repr(usize)" , I8 => "repr(i8)" , I16 => "repr(i16)" , I32 => "repr(i32)" , I64 => "repr(i64)" , I128 => "repr(i128)" , Isize => "repr(isize)" , }) , } , } } pub (crate) fn is_transparent (& self) -> bool { matches ! (self , Repr :: Transparent (_)) } pub (crate) fn is_c (& self) -> bool { use CompoundRepr :: * ; matches ! (self , Repr :: Compound (Spanned { t : C , span : _ } , _align)) } pub (crate) fn is_primitive (& self) -> bool { use CompoundRepr :: * ; matches ! (self , Repr :: Compound (Spanned { t : Primitive (_) , span : _ } , _align)) } pub (crate) fn get_packed (& self) -> Option < & Packed > { use AlignRepr :: * ; use Repr :: * ; if let Compound (_ , Some (Spanned { t : Packed (p) , span : _ })) = self { Some (p) } else { None } } pub (crate) fn get_align (& self) -> Option < Spanned < NonZeroU32 > > { use AlignRepr :: * ; use Repr :: * ; if let Compound (_ , Some (Spanned { t : Align (n) , span })) = self { Some (Spanned :: new (* n , * span)) } else { None } } pub (crate) fn is_align_gt_1 (& self) -> bool { self . get_align () . map (| n | n . t . get () > 1) . unwrap_or (false) } # [doc = " When deriving `Unaligned`, validate that the decorated type has no"] # [doc = " `#[repr(align(N))]` attribute where `N > 1`. If no such attribute exists"] # [doc = " (including if `N == 1`), this returns `Ok(())`, and otherwise it returns"] # [doc = " a descriptive error."] pub (crate) fn unaligned_validate_no_align_gt_1 (& self) -> Result < () , Error > { if let Some (n) = self . get_align () . filter (| n | n . t . get () > 1) { Err (Error :: new (n . span , "cannot derive `Unaligned` on type with alignment greater than 1" ,)) } else { Ok (()) } } }
    };
}

impl_34!()