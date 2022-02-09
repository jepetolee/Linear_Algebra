use std::fmt;
pub struct Matrix<T,C,R,S>{
    pub data: S,
    _phantoms: PhantomData<(T, R, C)>,
}
impl<T, R, C, S: fmt::Debug> fmt::Debug for Matrix<T, R, C, S> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.data.fmt(formatter)
    }
}

impl <T,R,C,S> Default for Matrix<T,R,C,S>
where
     T:scalar,


