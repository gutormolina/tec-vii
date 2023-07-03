fn main() {
	let idade = 23;

	match idade {
		0..=1 => { println!("Bebe") },
		2..=12 => { println!("Crianca") },
		13..=19 => { println!("Adolescente") },
		20..=29 => { println!("Jovem adulto") },
		30..=39 => { println!("Adulto jovem") },
		40..=59 => { println!("Adulto de meia-idade") },
		60.. => { println!("Idoso") },
	}
}
