enum Shape {
	Circulo(f64),
	Quadrado(f64),
	Triangulo(f64, f64),
	Retangulo(f64, f64),
	Nenhum
}

use Shape::*;

fn calcular_area(forma: Shape) {
	match forma {
        Circulo(x) => {
		    println!("Área do Círculo: {:?}", x.powf(2.0) * std::f64::consts::PI);
        },
        Quadrado(x) => {
            println!("Àrea do Quadrado: {:?}", x.powf(2.0));
        }
		Triangulo(x, y) => {
            println!("Área do Triângulo: {:?}", x * y / 2.0);
        }
        Retangulo(x, y) => {
            println!("Área do Retângulo: {:?}", x * y);
        }
        Nenhum => {
            println!("Forma geométrica inválida");
        }
    }
}

fn main() {
	let cir = Circulo(2.0);
    let qua = Quadrado(2.0);
    let tri = Triangulo(2.0, 4.0);
    let ret = Retangulo(2.0, 4.0);
    let nen = Nenhum;
	
    calcular_area(cir);
    calcular_area(qua);
    calcular_area(tri);
    calcular_area(ret);
    calcular_area(nen);
}
