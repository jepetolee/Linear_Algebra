#include "Matrix.h"
template<typename _Scalar>
Matrix<_Scalar>::Matrix(unsigned int _row, unsigned int _column) {
    this->matrix.resize(_row);
    for (int i = 0; i < matrix.size(); i++) {
        matrix[i].resize(_column);
    }
    this->rows = _row;
    this->columns = _column;

}