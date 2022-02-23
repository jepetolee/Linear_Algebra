#pragma once
namespace linearAlgebra {

	namespace Matrix {
		template <typename Scalar>
		class Matrix {
		private:
			vector<vector<Scalar>> Matrix;
		public:
			Matrix(unsigned int _rows,unsigned int _colums );


		};
	}
}
#include "Matrix.cpp"