namespace linearAlgebra {

	namespace Matrix {
		template <typename Mat>
		class Matrix1d {
		private:
			Mat _ghost = 1;
		public:

			Mat new_(int num) {
				Mat arr[num];
			}

			Matrix1d operator [] (int num) {
				Mat* arr = new Mat[num];
				return *this;
			}
		};
	}
}

using namespace linearAlgebra::Matrix;
int main() {
	Matrix1d<double> test1;
	constexpr int size = 3;
	test1[size];
	
}

