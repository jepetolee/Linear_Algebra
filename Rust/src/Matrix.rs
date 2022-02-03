use alloc::alloc::Global;
pub use core::alloc::Allocator;
enum VectorType{
    Integer(isize),
    UnsingedInteger(usize),
    Float(f64)
}

pub struct Matrix1D<T, A: Allocator = Global>{
}
impl <T> Matrix1D<T> {
    pub const fn new() -> Self{
        Matrix1D{

        }
    }

}


