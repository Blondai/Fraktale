use crate::complex::Complex;
use crate::complex_grid::ComplexGrid;
use crate::settings::Float;
use crate::iterator::Iterator;

mod complex;
mod complex_grid;
mod settings;
mod iterator;

fn main() {
    let mut complex_grid: ComplexGrid = ComplexGrid::from_boundaries(
        -1.4f32,
        1.4f32,
        10000usize,
        -1.4f32,
        1.4f32,
        10000usize,
    );
    let mut iterator: Iterator = Iterator::new(complex_grid, 500usize, fun);
    iterator.iterate();
    iterator.export();
    Iterator::python();
}

fn fun(value: Complex) -> Complex {
    const COMPLEX: Complex = Complex::const_new(-0.1, 0.651);
    value.pow(2) + COMPLEX
}
