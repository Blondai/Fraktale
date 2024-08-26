use std::fmt::format;
use crate::complex::Complex;
use crate::complex_grid::ComplexGrid;
use crate::settings::{Float, MAX};

pub struct Iterator {
    pub grid: ComplexGrid,
    iterations: usize,
    function: fn(Complex) -> Complex,
    changeable: Vec<bool>,
    pub brightness: Vec<Float>,
}

// New
impl Iterator {
    pub fn new(grid: ComplexGrid,
               iterations: usize,
               function: fn(Complex) -> Complex, ) -> Iterator {
        let changeable: Vec<bool> = vec![true; grid.len()];
        let brightness: Vec<Float> = vec![0 as Float; grid.len()];
        Iterator { grid, iterations, function, changeable, brightness }
    }
}

// Iterate
impl Iterator {
    pub fn iterate(self: &mut Iterator) {
        for iteration in 0..self.iterations {
            self.use_fun(iteration);
        }
    }

    fn use_fun(self: &mut Iterator, iteration: usize) {
        for index in 0..self.grid.len() {
            if !self.changeable[index] {
                continue;
            }

            let old_value: Complex = self.grid.values[index];

            if &old_value.len() > &MAX {
                self.brightness[index] = self.get_brightness(iteration);
                self.changeable[index] = false;
            }

            let new_value: Complex = (self.function)(old_value);
            self.grid.values[index] = new_value;
        }
    }

    fn get_brightness(self: &Iterator, iteration: usize) -> Float {
        1 as Float - (iteration as Float / self.iterations as Float)
    }
}

// Export
impl Iterator {
    pub fn export(self: Iterator) {
        self.export_data();

        self.export_information();
    }

    fn export_data(self: &Iterator) {
        use std::{fs::File, io::Write};

        let file_name_1: &str = "data.txt";
        let string_1: String = <Vec<f32> as Clone>::clone(&self.brightness)
            .into_iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let mut file_1: File = File::create(file_name_1).unwrap();
        file_1.write_all(string_1.as_bytes()).unwrap();
    }

    fn export_information(self: &Iterator) {
        use std::{fs::File, io::Write};

        let file_name: &str = "information.txt";
        let rows: String = format!("ROWS={}\n", self.grid.rows);
        let columns: String = format!("COLUMNS={}\n", self.grid.columns);
        let real_min: String = format!("REAL_MIN={}\n", self.grid.real_min);
        let real_max: String = format!("REAL_MAX={}\n", self.grid.real_max);
        let im_min: String = format!("IM_MIN={}\n", self.grid.im_min);
        let im_max: String = format!("IM_MAX={}\n", self.grid.im_max);
        let string: String = rows
            + &*columns
            + &*real_min
            + &*real_max
            + &*im_min
            + &*im_max;
        let mut file: File = File::create(file_name).unwrap();
        file.write_all(string.as_bytes()).unwrap();
    }
}

// Python
impl Iterator {
    pub fn python() {
        use std::process::Command;

        let output = Command::new("python")
            .arg("main.py")
            .output()
            .expect("Failed to execute Python script");
    }
}