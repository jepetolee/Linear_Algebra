#include "Matrix.h"
template<typename Scalar>
Matrix<Scalar>::Matrix(unsigned int _row, unsigned int _column,const Scalar& filler) {
    this->matrix.resize(_row);
    for (int i = 0; i < matrix.size(); i++) {
        matrix[i].resize(_column,_filler);
    }
    this->rows = _row;
    this->columns = _column;

}

template <typename _Scalar>
Matrix<_Scalar>::Matrix(const Matrix<_Scalar> & other) {
    this->matrix = other.matrix;
    this->rows = other.rows;
    this->columns = other.columns;

}

template<typename Scalar>
Matrix<Scalar>::~Matrix()
{
}
