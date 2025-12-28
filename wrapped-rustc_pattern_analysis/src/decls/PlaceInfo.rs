macro_rules! deps {
    () => {
        PatCx!();
        PlaceValidity!();
    };
}

macro_rules! PlaceInfo {
    () => {
        deps!();
        # [doc = " Data about a place under investigation. Its methods contain a lot of the logic used to analyze"] # [doc = " the constructors in the matrix."] struct PlaceInfo < Cx : PatCx > { # [doc = " The type of the place."] ty : Cx :: Ty , # [doc = " Whether the place is a private uninhabited field. If so we skip this field during analysis"] # [doc = " so that we don't observe its emptiness."] private_uninhabited : bool , # [doc = " Whether the place is known to contain valid data."] validity : PlaceValidity , # [doc = " Whether the place is the scrutinee itself or a subplace of it."] is_scrutinee : bool , }
    };
}

PlaceInfo!();