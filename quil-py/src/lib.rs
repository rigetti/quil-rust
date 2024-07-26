use pyo3::prelude::*;
use rigetti_pyo3::create_init_submodule;

pub mod expression;
pub mod instruction;
pub mod program;
pub mod validation;

create_init_submodule! {
    submodules: [
        "expression": expression::init_submodule,
        "instructions": instruction::init_submodule,
        "program": program::init_submodule,
        "validation": validation::init_submodule
    ],
}

#[pymodule]
fn quil(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    init_submodule("quil", py, m)?;
    Ok(())
}

pub fn init_quil_submodule(name: &str, py: Python<'_>, m: &PyModule) -> PyResult<()> {
    init_submodule(name, py, m)?;
    Ok(())
}

/// Implement `to_quil` and `to_quil_or_debug` methods for wrapper types whose inner type
/// implements [`Quil`](quil_rs::quil::Quil).
#[macro_export]
macro_rules! impl_to_quil {
    ($name: ident) => {
        #[pyo3::pymethods]
        impl $name {
            pub fn to_quil(&self) -> pyo3::PyResult<String> {
                quil_rs::quil::Quil::to_quil(rigetti_pyo3::PyWrapper::as_inner(self))
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
            }

            pub fn to_quil_or_debug(&self) -> String {
                quil_rs::quil::Quil::to_quil_or_debug(rigetti_pyo3::PyWrapper::as_inner(self))
            }
        }
    };
}

/// Implements pickling for an instruction by implementing __getstate__ and __reduce__.
///
/// The program is serialized using [`Quil`](quil_rs::quil::Quil), which means pickling can fail if
/// `to_quil()` raises an error (e.g. because the instruction contains placeholders).
///
/// To correctly implement __reduce__, an additional `_from_state` method is added to the class.
#[macro_export]
macro_rules! impl_pickle_for_instruction {
    ($name: ident) => {
        #[pyo3::pymethods]
        impl $name {
            // This will raise an error if the instruction contains any unresolved
            // placeholders. This is because they can't be converted to valid quil,
            // nor can they be serialized and deserialized in a consistent
            // way.
            pub fn __getstate__(
                &self,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
                use pyo3::IntoPy;
                Ok(pyo3::types::PyBytes::new(py, self.to_quil()?.as_bytes()).into_py(py))
            }

            // __reduce__ must return a tuple containing the necessary components to successfully
            // construct a class instance.
            //
            // In this case, we initialize a new instance using _fromstate with the state
            // generateed by __getstate__.
            //
            // See [Python's __reduce__ documentation](https://docs.python.org/3/library/pickle.html#object.__reduce__)
            fn __reduce__<'py>(
                &'py self,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<&'py pyo3::PyAny> {
                use pyo3::IntoPy;
                let callable = py.get_type::<Self>().getattr("_from_state")?;
                let state = self.__getstate__(py)?;
                let args = pyo3::types::PyTuple::new(py, &[state.into_py(py)]);
                Ok(pyo3::types::PyTuple::new(py, &[callable, args]))
            }

            // __reduce__ must return a callable with any necessary arguments to initialize the
            // class instance. This is often done with __new__, but because we define class
            // specific parameters in __new__, and field access patterns differ between
            // instruction types (e.g. enums versus data structs), that can't be specified in a
            // generic enough way for this macro. As an alternative, we use the serialized state
            // from __getstate__ to initialize a copy of the instance.
            #[staticmethod]
            pub fn _from_state(
                py: pyo3::Python<'_>,
                state: &pyo3::types::PyBytes,
            ) -> pyo3::PyResult<Self> {
                let input = std::str::from_utf8(state.as_bytes())?;
                let instruction = $crate::instruction::PyInstruction::parse(input)?;
                instruction.inner(py)?.extract(py)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_eq {
    ($name: ident) => {
        #[pyo3::pymethods]
        impl $name {
            pub fn __richcmp__(
                &self,
                py: pyo3::Python<'_>,
                other: &Self,
                op: pyo3::pyclass::CompareOp,
            ) -> pyo3::PyObject {
                use pyo3::IntoPy;
                match op {
                    pyo3::pyclass::CompareOp::Eq => (self == other).into_py(py),
                    pyo3::pyclass::CompareOp::Ne => (self != other).into_py(py),
                    _ => py.NotImplemented(),
                }
            }
        }
    };
}
