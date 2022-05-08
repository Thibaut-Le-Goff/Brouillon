fn main() {
	use rand::{thread_rng, Rng};
	//use rand::distributions::uniform;
	//impl UniformSample for UniformFloat<f64>;

	//let mut m: [f64; 4] = [0.0; 4];
	let mut m = [[0.0; 2], [0.0; 2], [0.0; 2], [0.0; 2]];
	let mut rng = thread_rng();

	for x in 0..= 3 {
		for y in 0..= 1 {
			//m[x][y] = alea.gen_range::<f64, bool>(0.0..1.0);
			//let mut saut: f64 = rng.gen_range(0.0..=1.0);
			let saut: f64 = rng.gen_range(0.0..=1.0);
			m[x][y] = saut;
		}
	}

	println!("La matrice des poids est :");
	for x in 0..= 3 {
		println!("{:?}", m[x]); 
	}
/*
	let mut v: [f64; 3] = [1.0, 2.0, 3.0]; 
	println!("Le vecteur des inputs est : {:?}", v);

	let mut a: [f64; 4] = [0.0; 4];
	let mut t: f64;

	for x in 0..= 3 {
		// Pour chacune des ligne de la matrice...
		t = 0.0;
		// Met la valeur de z à 0 afin que le
		// z d'une ligne vienne s'incrémenté 
		// à celui d'une autre ligne.
		for y in 0..= 1 {
		// Pour chacune des cases de la ligne...
			t = (v[y] * m[x][y]) + t;
			// réalise le calcule matriciel, 
			// l'aditionne au précedent et
			// le met dans z.
		}
		a[x] = t;   
		// met z dans dans la matrice a à la position x.
	}
	println!("La multiplication de la matrice par le vecteur donne : {:?}", a);
*/
}
