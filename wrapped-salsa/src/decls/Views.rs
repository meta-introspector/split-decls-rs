macro_rules! deps {
    () => {
        ViewCaster!();
    };
}

macro_rules! Views {
    () => {
        deps!();
        # [doc = " A `Views` struct is associated with some specific database type"] # [doc = " (a `DatabaseImpl<U>` for some existential `U`). It contains functions"] # [doc = " to downcast to `dyn DbView` for various traits `DbView` via this specific"] # [doc = " database type."] # [doc = " None of these types are known at compilation time, they are all checked"] # [doc = " dynamically through `TypeId` magic."] pub struct Views { source_type_id : TypeId , view_casters : boxcar :: Vec < ViewCaster > , }
    };
}

Views!();