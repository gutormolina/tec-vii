struct Ponto { x: i32, y: i32 }

struct Retangulo {
	p1: Ponto,
	p2: Ponto,
	area: i32
}

impl Ponto {
	fn cria(x: i32, y: i32) -> Ponto { Ponto { x, y } }
	fn mover(&mut self, dx: i32, dy: i32) { self.x += dx; self.y += dy; }
}

impl Retangulo {
	fn cria(p1: Ponto, p2: Ponto, area: i32) -> Retangulo { Retangulo { p1, p2, area} }
	fn calcula_area(&mut self, pb: Ponto, pa: Ponto) { self.area = (pb.x + pb.y) * (pa.x + pa.y); }
}

fn main() {
	let mut ponto_ie = Ponto::cria(10,20);
	println!("Ponto IE: {:?}", ponto_ie);
	let mut ponto_sd = Ponto::cria(30,40);
	println!("Ponto SD: {:?}", ponto_sd);
	let mut area = Retangulo::calcula_area(ponto_ie, ponto_sd);
	let mut retangulo = Retangulo::cria(ponto_ie, ponto_sd, area);
	println!("Area: {:?}", area); 
}
