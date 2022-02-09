
namespace linearAlgebra {
	
	namespace Matrix {

		template <typename Mat>
		class Matrix1d {
		private:
			Mat _data[];
		public:
			
			Mat new_(int num){
				_data[num];
			}

			Matrix1d operator [] (int num) {
				_data[num];
			}
		};
	}
}

using namespace linearAlgebra::Matrix;
int main() {
	Matrix1d<double> test1;

	test1[3];
}

