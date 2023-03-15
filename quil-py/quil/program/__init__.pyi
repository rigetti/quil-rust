from typing import Dict, Set, final, List, Optional

from quil.instructions import (
    AttributeValue,
    Calibration,
    Declaration,
    FrameIdentifier,
    GateDefinition,
    MeasureCalibrationDefinition,
    Measurement,
    Instruction,
    Gate,
    Qubit,
    Vector,
    Waveform,
)

@final
class Program:
    @staticmethod
    def __new__(cls) -> "Program": ...
    @property
    def instructions(self) -> List[Instruction]: ...
    @property
    def calibrations(self) -> CalibrationSet: ...
    @property
    def waveforms(self) -> Dict[str, Waveform]: ...
    @property
    def frames(self) -> FrameSet: ...
    @property
    def memory_regions(self) -> Dict[str, MemoryRegion]: ...
    @property
    def declarations(self) -> Dict[str, Declaration]: ...
    @property
    def defined_gates(self) -> List[GateDefinition]: ...
    def dagger(self) -> "Program":
        """
        Creates a new conjugate transpose of the ``Program`` by reversing the order of gate
        instructions and applying the DAGGER modifier to each.

        Raises a ``GateError`` if any of the instructions in the program are not a ``Gate`
        """
        ...
    def expand_calibrations(self) -> "Program":
        """
        Expand any instructions in the program which have a matching calibration, leaving the others
        unchanged. Recurses though each instruction while ensuring there is no cycle in the expansion
        graph (i.e. no calibration expands directly or indirectly into itself)
        """
        ...
    def into_simplified(self) -> "Program":
        """
        Simplify this program into a new [`Program`] which contains only instructions
        and definitions which are executed; effectively, perform dead code removal.

        Removes:
        - All calibrations, following calibration expansion
        - Frame definitions which are not used by any instruction such as `PULSE` or `CAPTURE`
        - Waveform definitions which are not used by any instruction

        When a valid program is simplified, it remains valid.
        """
        ...
    def get_used_qubits(self) -> Set[Qubit]:
        """
        Returns a set consisting of every Qubit that is used in the program.
        """
        ...
    def add_instruction(self, instruction: Instruction):
        """
        Add an instruction to the end of the program.
        """
        ...
    def add_instructions(self, instruction: List[Instruction]):
        """
        Add a list of instructions to the end of the program.
        """
        ...
    @staticmethod
    def parse(quil: str) -> "Program":
        """
        Parses the given Quil string and returns a new ``Program``.
        Raises a ``ProgramError`` if the given string isn't valid Quil.
        """
    def to_instructions(self, include_headers: bool) -> List[Instruction]: ...
    def to_headers(self) -> List[Instruction]: ...

@final
class CalibrationSet:
    @staticmethod
    def __new__(
        cls,
        calibrations: List[Calibration],
        measure_calibration_definitions: List[MeasureCalibrationDefinition],
    ) -> "CalibrationSet": ...
    @property
    def calibrations(self) -> List[Calibration]: ...
    @property
    def measure_calibrations(self) -> List[MeasureCalibrationDefinition]: ...
    def expand(
        self, instruction: Instruction, previous_calibrations: List[Instruction]
    ) -> List[Instruction]:
        """
        Given an instruction, return the instructions to which it is expanded if there is a match.
        Recursively calibrate instructions, returning an error if a calibration directly or indirectly
        expands into itself.
        """
    ...

    def get_match_for_measurement(
        self, measurement: Measurement
    ) -> Optional[MeasureCalibrationDefinition]:
        """
        Returns the last-specified ``MeasureCalibrationDefinition`` that matches the target
        qubit (if any), or otherwise the last-specified one that specified no qubit.
        """
        ...
    def get_match_for_gate(self, gate: Gate) -> Optional[Calibration]:
        """
        Return the final calibration which matches the gate per the QuilT specification:

        A calibration matches a gate if:
        1. It has the same name
        2. It has the same modifiers
        3. It has the same qubit count (any mix of fixed & variable)
        4. It has the same parameter count (both specified and unspecified)
        5. All fixed qubits in the calibration definition match those in the gate
        6. All specified parameters in the calibration definition match those in the gate
        """
    def __len__(self) -> int: ...
    def is_empty(self) -> bool:
        """Returns ``True`` if the ``CalibrationSet`` contains no data."""
        ...
    def push_calibration(self):
        """
        Add another gate ``Calibration`` (`DEFCAL`) to the set.
        """
        ...
    def push_measurement_calibration(self):
        """
        Add another ``MeasureCalibrationDefinition`` (`DEFCAL MEASURE`) to the set
        """
    def extend(self):
        """
        Append another [`CalibrationSet`] onto this one
        """
        ...
    def to_instructions(self):
        """
        Return the Quil instructions which describe the contained calibrations
        """
        ...

@final
class FrameSet:
    @staticmethod
    def __new__(cls) -> "FrameSet": ...
    def get_all_frames(self) -> Dict[FrameIdentifier, Dict[str, AttributeValue]]: ...

class MemoryRegion:
    @staticmethod
    def __new__(cls, size: Vector, sharing: Optional[str]) -> "MemoryRegion": ...
    @property
    def size(self) -> Vector: ...
    @size.setter
    def size(self, size: Vector): ...
    @property
    def sharing(self) -> Optional[str]: ...
    @sharing.setter
    def sharing(self, sharing: Optional[str]): ...
