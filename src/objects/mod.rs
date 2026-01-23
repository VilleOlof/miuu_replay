pub mod bumper;
pub mod elevator;
pub mod marble;
pub mod powerup;

pub(crate) mod field_macro {
    #[macro_export]
    macro_rules! field {
        ($fn_name:ident, &$return_name:ty, $field_str:expr, $rewind_type:ident) => {
            pub fn $fn_name(&self) -> Result<&$return_name, crate::ReplayError> {
                let val = self
                    .inner
                    .data
                    .iter()
                    .find(|d| d.text == $field_str)
                    .ok_or(crate::ReplayError::MissingField($field_str))?;

                match &val.curve {
                    crate::rewind_curve::IRewindCurve::$rewind_type(v) => Ok(v),
                    _ => Err(crate::ReplayError::MismatchedCurveTypes {
                        lhs: crate::RewindCurveType::$rewind_type,
                        rhs: val.rewind_type,
                    }),
                }
            }
        };
    }
}
