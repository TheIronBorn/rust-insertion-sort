fn main() {

	//let mut array_A = [31i, 41, 59, 26, 41, 58];
	let mut array_A = [5i,2,4,6,1,3];
	let mut i: int;
	let mut key: int;


	// It is not necessary to explicitly declare j
	for j in range(1u, array_A.len()) { //arrays only work with uint (unsigned)
		key = array_A[j];
		i = j as int - 1;

		while i >= 0 && array_A[i as uint] > key {
			array_A[i as uint + 1] = array_A[i as uint];
			i = i - 1;
		};
		array_A[i as uint+1]  = key;
		
	};


	//Just for printing purposes
	for j in range(0u, array_A.len()) {
		println!("{}", array_A[j]);
	};

    
}