fn main() {

	let mut array_A = [31i, 41, 59, 26, 41, 58];
	//let mut array_A = [5i,2,4,6,1,3];
	let mut i: int;

	//utility variable. Uggly hack to avoid casting back and foward from int to uint
	let mut i_u: uint; 
	let mut key: int;


	// It is not necessary to explicitly declare j
	for j in range(1u, array_A.len()) { //arrays only work with uint (unsigned)
		key = array_A[j];
		i_u = j - 1;
		i = i_u as int;

		while i >= 0 && array_A[i_u] > key {
			array_A[i_u + 1] = array_A[i_u];

			i = i - 1;
			i_u = i as uint;
		};
		array_A[i_u + 1]  = key;
		
	};


	//Just for printing purposes
	for j in range(0u, array_A.len()) {
		println!("{}", array_A[j]);
	};

    
}