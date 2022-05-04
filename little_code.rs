fn main() {
	let m  = [[1; 3], [2; 3], [3; 3]];
    // la matrice est sous la forme :
    //  | 1  1  1 |
    //  | 2  2  2 |
    //  | 3  3  3 |

	 let v: [u16; 3] = [4; 3];
    // le vecteur est sous la forme :
    // | 4  4  4 |
    //
    // au lieux d'être sous la forme :
    // | 4 |
    // | 4 |
    // | 4 |

    // Multiplication de la matrice m par le vecteur v :
    // entrainement :
	println!("la position [1][0] est : {:?}", m[1][0]);
	println!("la position [2][1] est : {:?}", m[2][1]);
	let test: u16 = m[1][0] * m[2][0];
	println!("multiplication de [1][0] par [2][0] donne {:?}", test);
	println!("La multiplication de la matrice : \n {:?}", m);
	println!("Par le vecteur : \n {:?}", v);

    // application :
	let mut a: [u16; 3] = [0; 3];
	let mut z: u16;
	
	for x in 0..= 2 {
        // Pour chacune des ligne de la matrice...
        z = 0;
        // Met la valeur de z à 0 afin que le
        // z d'une ligne vienne s'incrémenté 
        // à celui d'une autre ligne.
		for y in 0..= 2 {
        // Pour chacune des cases de la ligne...
			z = (v[y] * m[x][y]) + z;
            // réalise le calcule matriciel, 
            // l'aditionne au précedent et
            // le met dans z.
		}
	    a[x] = z;	
        // met z dans dans la matrice a à la position x.
    }

    println!("Donne le vecteur : \n {:?}",a);
}
