use std::collections::BTreeMap;

use super::diagram::wire::{QuantikzColumn, Wire};

/// RenderSettings contains the metadata that allows the user to customize how
/// the circuit is rendered or use the default implementation.
#[derive(Clone, Copy, Debug)]
pub struct RenderSettings {
    /// Convert numerical constants, e.g. pi, to LaTeX form.
    pub texify_numerical_constants: bool,
    /// Include all qubits implicitly referenced in the Quil program.
    pub impute_missing_qubits: bool,
    /// Label qubit lines.
    pub label_qubit_lines: bool,
}

impl Default for RenderSettings {
    /// Returns the default RenderSettings.
    fn default() -> Self {
        Self {
            /// false: pi is π.
            texify_numerical_constants: true,
            /// true: `CNOT 0 2` would have three qubit lines: 0, 1, 2.
            impute_missing_qubits: false,
            /// false: remove Lstick/Rstick from latex.
            label_qubit_lines: true,
        }
    }
}

impl RenderSettings {
    /// Adds missing qubits between the first qubit and last qubit in a
    /// diagram's circuit. If a missing qubit is found, a new wire is created
    /// and pushed to the diagram's circuit.
    ///
    ///  # Arguments
    /// `last_column` - total number of instructions from Program
    /// `circuit` - the circuit of the diagram
    ///
    /// # Examples
    /// ```
    /// use quil_rs::{Program, program::latex::settings::RenderSettings};
    /// use std::str::FromStr;
    /// let program = Program::from_str("H 0\nCNOT 0 1").expect("");
    /// let settings = RenderSettings {
    ///     impute_missing_qubits: true,
    ///     ..Default::default()
    /// };
    /// program.to_latex(settings).expect("");
    /// ```
    pub(crate) fn impute_missing_qubits(
        last_column: usize,
        circuit: &mut BTreeMap<u64, Box<Wire>>,
    ) {
        let mut keys_iter = circuit.keys();

        // get the first qubit in the BTreeMap
        let Some(first) = keys_iter
            .next()
            .map(|wire| wire + 1) else { return; };

        // get the last qubit in the BTreeMap
        let Some(last) = keys_iter
            .last()
            .map(|wire| wire - 1) else { return; };

        // search through the range of qubits
        for qubit in first..=last {
            // if the qubit is not found impute it
            circuit.entry(qubit).or_insert_with(|| {
                let mut wire = Wire {
                    ..Default::default()
                };

                // insert empties based on total number of columns
                for _ in 0..last_column {
                    wire.columns.push(QuantikzColumn::default());
                }

                Box::new(wire)
            });
        }
    }
}