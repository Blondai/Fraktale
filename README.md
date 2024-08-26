## How it started
This was a project that started in August 2021 and was part of a group work in my math degree.
The Goal was to implement functions to generate Julia sets ([Wikipedia](https://en.wikipedia.org/wiki/Julia_set))
The first version which you can find under 'Julia Fraktale.py' was part of this project and also has a PDF file which describes to process in german.
I must confess that this version is very slow, not at all efficient and not written to any [PEP 8](https://peps.python.org/pep-0008/) standards.
Still the generated pictures are breathtaking.

<p align="center">
  <img src="./Pictures/Mandelbrot UHD.jpg" width="600px">
</p>

With a little bit of Photoshop magic you can get

<p align="center">
  <img src="./Pictures/Mandelbrot edit.png" width="600px">
</p>

which has been my lockscreen background since then.
You can find more (un)edited pictures in 'Pictures' folder.

## How it went
Later in early 2023 I revisited this project to get a more efficient Code.
I timed different methods of applying a function to a part of an NumPy array to find the most efficient way.
This resulted in 'GenerateGrid.py', 'Iterate.py', 'Visualize.py' and 'main.py'.
I think with NumPy and Python you can't get any faster.

## How it's going
To get an even faster implementation of this Julia set stuff I also made a Rust implementation which is indeed blazingly fast.
The following 10000 x 10000 picture was generated with the Rust Code.

<p align="center">
  <img src="./Pictures/Rust.png" width="600px">
</p>

If you want to use the Rust version you should consider using `cargo build --release`.
The faster calculation easily overtakes the longer compile time.

## Goal
The dream goal of this project would be an implementation where you can also zoom into the picture to get a clearer view of different parts of the fractal.