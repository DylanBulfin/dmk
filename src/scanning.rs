//! This module contains the logic for translating the physical state of controller pins into a
//! PhysicalKeyboard. In most mainstream full-size keyboards matrix scanning is used, but in
//! smaller keyboards like macropads or numpads there may actually be a 1-1 key-pin correspondence.

use crate::{
    controller::{MAX_PINS, PinSet},
    physical_layout::MAX_KEYS,
    timer::{Duration, Timer},
};

pub trait ScanAlgorithm {
    fn configure_pins(&self, pins: &mut impl PinSet);
    fn scan_pins(&mut self, pins: &mut impl PinSet, timer: &impl Timer) -> [bool; MAX_KEYS];
}

#[derive(Debug, PartialEq)]
pub enum MatrixScanDirection {
    /// Rows are outputs, and columns are inputs. Equivalent of col2row in ZMK
    OutputRows,
    /// Columns are outputs, and rows are inputs. Equivalent of row2col in ZMK
    OutputCols,
}

pub struct MatrixScanAlgorithm {
    pub direction: MatrixScanDirection,
    pub row_pins: [Option<usize>; MAX_PINS],
    pub col_pins: [Option<usize>; MAX_PINS],
}

impl ScanAlgorithm for MatrixScanAlgorithm {
    fn configure_pins(&self, pins: &mut impl PinSet) {
        let mut i = 0;
        match self.direction {
            MatrixScanDirection::OutputRows => {
                while let Some(row) = self.row_pins[i] {
                    pins.set_pin_output(row);
                    i += 1
                }
                while let Some(col) = self.col_pins[i] {
                    pins.set_pin_input(col);
                    i += 1
                }
            }
            MatrixScanDirection::OutputCols => {
                while let Some(col) = self.row_pins[i] {
                    pins.set_pin_output(col);
                    i += 1
                }
                while let Some(row) = self.col_pins[i] {
                    pins.set_pin_input(row);
                    i += 1
                }
            }
        }
    }

    fn scan_pins(&mut self, pins: &mut impl PinSet, timer: &impl Timer) -> [bool; MAX_KEYS] {
        let mut keys = [false; _];
        let mut key = 0;
        let mut i = 0;

        while let Some(output) = self.get_output_pin_collection()[i] {
            pins.set_pin_state(output, false);
            i += 1;
        }

        i = 0;
        while let Some(output) = self.get_output_pin_collection()[i] {
            pins.set_pin_state(output, true);

            // Wait for debounce
            timer.wait(Duration::new(10));

            let mut j = 0;
            while let Some(input) = self.get_input_pin_collection()[j] {
                keys[key] = pins.get_pin_state(input);
                key += 1;

                j += 1;
            }

            i += 1
        }

        keys
    }
}

impl MatrixScanAlgorithm {
    fn get_input_pin_collection(&self) -> &[Option<usize>; MAX_PINS] {
        match self.direction {
            MatrixScanDirection::OutputRows => &self.col_pins,
            MatrixScanDirection::OutputCols => &self.row_pins,
        }
    }

    fn get_output_pin_collection(&self) -> &[Option<usize>; MAX_PINS] {
        match self.direction {
            MatrixScanDirection::OutputRows => &self.row_pins,
            MatrixScanDirection::OutputCols => &self.col_pins,
        }
    }
}
