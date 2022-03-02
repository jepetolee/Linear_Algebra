#ifndef MATRIX_H
#define MATRIX_H

		template <typename Scalar>
		class Matrix {
		private:
			vector<vector<Scalar>> Matrix;
		public:
			Matrix(unsigned int _rows,unsigned int _colums,const Scalar& fill );
			Matrix(const Matrix<Scalar>& other);

			virtual ~Matrix();


		};


#include "Matrix.cpp"
#endif