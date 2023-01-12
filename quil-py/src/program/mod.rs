use std::collections::{BTreeMap, HashMap};

use pyo3::{
    create_exception,
    exceptions::PyRuntimeError,
    prelude::*,
    types::{PyList, PyString},
};

use quil_rs::{instruction::Instruction, Program};
use rigetti_pyo3::{impl_repr, py_wrap_struct, PyWrapper, PyWrapperMut, ToPython};

use crate::instruction::{
    declaration::PyDeclaration, gate::PyGateDefinition, memory_region::PyMemoryRegion,
    waveform::PyWaveform, PyInstruction,
};

use self::{calibration_set::PyCalibrationSet, frame::PyFrameSet};

pub mod calibration_set;
pub mod frame;

create_exception!(quil, ParseError, PyRuntimeError);

// may need to define constructors "by hand", instead of imported macro
// gives full control
py_wrap_struct! {
    PyProgram(Program) as "Program" {
        py -> rs {
            string: Py<PyString> => Program {
                let native_program = string
                    .as_ref(py)
                    .to_str()?
                    .parse::<quil_rs::Program>()
                    .map_err(|e| ParseError::new_err(e.to_string()))?;
                Ok::<_, PyErr>(native_program)
            }
        },
        rs -> py {
            program: Program => Py<PyString> { program.to_string(true).to_python(py) }
        }
    }
}
impl_repr!(PyProgram);

#[pymethods]
impl PyProgram {
    #[getter]
    pub fn instructions<'a>(&self, py: Python<'a>) -> PyResult<&'a PyList> {
        Ok(PyList::new(
            py,
            self.as_inner()
                .instructions
                .iter()
                .map(|i| i.to_python(py))
                .collect::<PyResult<Vec<PyInstruction>>>()?,
        ))
    }

    #[getter]
    pub fn calibrations(&self, py: Python<'_>) -> PyResult<PyCalibrationSet> {
        self.as_inner().calibrations.to_python(py)
    }

    #[getter]
    pub fn waveforms(&self, py: Python<'_>) -> PyResult<BTreeMap<String, PyWaveform>> {
        self.as_inner().waveforms.to_python(py)
    }

    #[getter]
    pub fn frames(&self, py: Python<'_>) -> PyResult<PyFrameSet> {
        self.as_inner().frames.to_python(py)
    }

    #[getter]
    pub fn memory_regions(&self, py: Python<'_>) -> PyResult<BTreeMap<String, PyMemoryRegion>> {
        self.as_inner()
            .memory_regions
            .iter()
            .map(|(name, memory_region)| Ok((name.to_python(py)?, memory_region.to_python(py)?)))
            .collect()
    }

    #[getter]
    // TODO: Should this filtering move to Program? Should we assume memory_regions will always make up all
    // declarations and simplify this?
    pub fn declarations(&self, py: Python<'_>) -> PyResult<HashMap<String, PyDeclaration>> {
        self.as_inner()
            .to_instructions(true)
            .iter()
            // TODO: Is there some clever and still readable way to consolidate ths into one filter map?
            .filter_map(|inst| match inst {
                Instruction::Declaration(declaration) => Some(declaration),
                _ => None,
            })
            .map(|declaration| Ok((declaration.name.clone(), declaration.to_python(py)?)))
            .collect()
    }

    #[getter]
    pub fn defined_gates(&self, py: Python<'_>) -> PyResult<Vec<PyGateDefinition>> {
        self.as_inner()
            .to_instructions(true)
            .iter()
            .filter_map(|inst| match inst {
                Instruction::GateDefinition(gate_def) => Some(gate_def.to_python(py)),
                _ => None,
            })
            .collect()
    }

    pub fn expand_calibrations(&self) -> PyResult<Self> {
        self.as_inner()
            .expand_calibrations()
            .map_err(|e| ParseError::new_err(e.to_string()))
            .map(PyProgram::from)
    }

    pub fn into_simplified(&self) -> PyResult<Self> {
        self.as_inner()
            .into_simplified()
            .map_err(|e| ParseError::new_err(e.to_string()))
            .map(PyProgram::from)
    }

    pub fn add_instruction(&mut self, instruction: PyInstruction) {
        self.as_inner_mut().add_instruction(instruction.into())
    }

    pub fn __str__(&self) -> PyResult<Py<PyString>> {
        self.clone().try_into()
    }

    pub fn __add__(&self, py: Python<'_>, rhs: Self) -> PyResult<Self> {
        let new = self.as_inner() + rhs.as_inner();
        new.to_python(py)
    }
}
