enum Shape {
	Circulo(f64),
	Quadrado(f64),
	Triangulo(f64, f64),
	Retangulo(f64, f64),
	Nenhum
}

use Shape::*;

fn calcular_area(forma: Shape) {
	match forma = Shape.Circulo {
		println!("Área do Círculo: {:?}", forma * std::f64::consts::PI * 2);
			
}

fn main() {
	let formaGeom = Shape::Circulo(2.0);
	calcular_area(formaGeom);
}
