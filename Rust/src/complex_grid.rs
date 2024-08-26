use crate::complex::Complex;
use crate::settings::Float;

pub struct ComplexGrid {
    pub values: Vec<Complex>,
    pub rows: usize,
    pub columns: usize,
    pub real_min: Float,
    pub real_max: Float,
    pub im_min: Float,
    pub im_max: Float,
}

// New
impl ComplexGrid {
    pub fn new(values: Vec<Complex>,
               rows: usize,
               columns: usize,
               real_min: Float,
               real_max: Float,
               im_min: Float,
               im_max: Float, ) -> ComplexGrid {
        ComplexGrid {
            values,
            rows,
            columns,
            real_min,
            real_max,
            im_min,
            im_max,
        }
    }

    pub fn from_boundaries(
        real_min: Float,
        real_max: Float,
        real_num: usize,
        im_min: Float,
        im_max: Float,
        im_num: usize,
    ) -> ComplexGrid {
        let pair: (Vec<Float>, Vec<Float>) = ComplexGrid::get_parts(
            real_min,
            real_max,
            real_num,
            im_min,
            im_max,
            im_num,
        );
        let reals: Vec<Float> = pair.0;
        let imaginaries: Vec<Float> = pair.1;
        let mut complexes: Vec<Complex> = Vec::new();
        for real in &reals {
            for imaginary in &imaginaries {
                complexes.push(Complex::new(*real, *imaginary))
            }
        }
        ComplexGrid::new(complexes, real_num, im_num, real_min, real_max, im_min, im_max)
    }

    fn get_parts(real_min: Float,
                 real_max: Float,
                 real_num: usize,
                 im_min: Float,
                 im_max: Float,
                 im_num: usize,
    ) -> (Vec<Float>, Vec<Float>) {
        let mut reals: Vec<Float> = Vec::new();
        let real_distance: Float = (real_max - real_min) / (real_num - 1) as Float;
        let mut real_part: Float = real_min;

        for _ in 0..real_num {
            reals.push(real_part);
            real_part += real_distance
        }

        let mut imaginaries: Vec<Float> = Vec::new();
        let imaginary_distance: Float = (im_max - im_min) / (im_num - 1) as Float;
        let mut imaginary_part: Float = im_min;

        for _ in 0..im_num {
            imaginaries.push(imaginary_part);
            imaginary_part += imaginary_distance
        }

        (reals, imaginaries)
    }
}

// Indexing and Values
impl ComplexGrid {
    fn index(self: &ComplexGrid,
             row: usize,
             column: usize,
    ) -> usize {
        row * self.columns + column
    }

    pub fn value(self: &ComplexGrid,
                 row: usize,
                 column: usize,
    ) -> Complex {
        let index: usize = *&self.index(row, column);
        self.values[index].clone()
    }

    pub fn change(mut self: &mut ComplexGrid,
                  row: usize,
                  column: usize,
                  new_value: Complex,
    ) {
        let index: usize = *&self.index(row, column);
        self.values[index] = new_value;
    }
}

impl ComplexGrid {
    pub fn len(self: &ComplexGrid) -> usize {
        self.rows * self.columns
    }
}
