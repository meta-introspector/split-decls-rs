macro_rules! deps {
    () => {
        ValueAbi!();
        FieldsShape!();
        VariantsShape!();
        Size!();
        Align!();
    };
}

macro_rules! LayoutShape {
    () => {
        deps!();
        # [doc = " The layout of a type in memory."] # [derive (Clone , Debug , PartialEq , Eq , Hash , Serialize)] pub struct LayoutShape { # [doc = " The fields location within the layout"] pub fields : FieldsShape , # [doc = " Encodes information about multi-variant layouts."] # [doc = " Even with `Multiple` variants, a layout still has its own fields! Those are then"] # [doc = " shared between all variants."] # [doc = ""] # [doc = " To access all fields of this layout, both `fields` and the fields of the active variant"] # [doc = " must be taken into account."] pub variants : VariantsShape , # [doc = " The `abi` defines how this data is passed between functions."] pub abi : ValueAbi , # [doc = " The ABI mandated alignment in bytes."] pub abi_align : Align , # [doc = " The size of this layout in bytes."] pub size : Size , }
    };
}

LayoutShape!()