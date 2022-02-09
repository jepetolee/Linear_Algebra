extern crate alloc;

mod Matrix;
mod allocator;

use Matrix::Matrix1D;

fn main() {
    let matrix:Matrix1D<isize> = Matrix1D::new();
    println!("{}",matrix);
    let vec = Vec::new();
}