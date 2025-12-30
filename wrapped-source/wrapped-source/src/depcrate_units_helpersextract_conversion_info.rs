// Generated macro for extract_conversion_info (function)
macro_rules! Depcrate_units_helpersextract_conversion_info {
() => {
// Module: crate::units::helpers
// Provides: {"extract_conversion_info"}
// Dependencies: {}
# [doc = " Extracts the conversion info from a base unit, factor and offset."] pub (crate) fn extract_conversion_info < 'data > (unit_id : UnitID , base_unit : & str , factor : & ScientificNumber , offset : & ScientificNumber ,) -> Result < ConversionInfo < 'data > , DataError > { let factor_fraction = convert_slices_to_fraction (& factor . clean_num , & factor . clean_den) ? ; let offset_fraction = convert_slices_to_fraction (& offset . clean_num , & offset . clean_den) ? ; let (factor_num , factor_den , factor_sign) = flatten_fraction (factor_fraction) ; let (offset_num , offset_den , offset_sign) = flatten_fraction (offset_fraction) ; let exactness = if factor . exactness == Exactness :: Exact && offset . exactness == Exactness :: Exact { Exactness :: Exact } else { Exactness :: Approximate } ; let base_unit = MeasureUnit :: try_from_str (base_unit) . map_err (| _ | DataError :: custom ("the base unit is not valid")) ? ; Ok (ConversionInfo { unit_id , basic_units : ZeroVec :: from_iter (base_unit . single_units () . iter () . copied ()) , factor_num : factor_num . into () , factor_den : factor_den . into () , factor_sign , offset_num : offset_num . into () , offset_den : offset_den . into () , offset_sign , exactness , }) }
};
}
