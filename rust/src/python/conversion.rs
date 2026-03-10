use pyo3::conversion::FromPyObject;
use pyo3::prelude::*;
use pyo3::types::PyAny;

use crate::algorithms::rv::{DomainType, FunctionalForm, Number};
use num_rational::Rational64;

/// Ensures Python lists that could contain floats, integers
/// or rational numbers are correctly cast into the Number enum
impl<'py> FromPyObject<'py> for Number {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(int) = obj.extract::<i64>() {
            return Ok(Number::Integer(int));
        }

        if let Ok(float) = obj.extract::<f64>() {
            return Ok(Number::Float(float));
        }

        if obj.hasattr("p")? & obj.hasattr("q")? {
            let numerator: i64 = obj.getattr("p")?.extract()?;
            let denominator: i64 = obj.getattr("q")?.extract()?;
            let rational = Rational64::new(numerator, denominator);
            return Ok(Number::Rational(rational));
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Expected int, float, or sympy.Rational",
        ))
    }
}

impl<'py> FromPyObject<'py> for FunctionalForm {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        let s: String = obj.extract()?;
        match s.as_str() {
            "cdf" => Ok(FunctionalForm::Cdf),
            "chf" => Ok(FunctionalForm::Chf),
            "hf" => Ok(FunctionalForm::Hf),
            "idf" => Ok(FunctionalForm::Idf),
            "pdf" => Ok(FunctionalForm::Pdf),
            "sf" => Ok(FunctionalForm::Sf),
            _ => Err(pyo3::exceptions::PyValueError::new_err(
                "Invalid FunctionalForm",
            )),
        }
    }
}

impl<'py> FromPyObject<'py> for DomainType {
    fn extract_bound(obj: &Bound<'py, PyAny>) -> PyResult<Self> {
        let s: String = obj.extract()?;
        match s.as_str() {
            "continuous" => Ok(DomainType::Continuous),
            "discrete" => Ok(DomainType::Discrete),
            "Discrete" => Ok(DomainType::DiscreteFunctional),
            _ => Err(pyo3::exceptions::PyValueError::new_err(
                "Invalid DomainType",
            )),
        }
    }
}
