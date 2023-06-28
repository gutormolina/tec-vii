use rand::Rng;
use std::time::Instant;

fn gera_vet(size: usize) -> Vec<i32> {
	let mut rng = rand::thread_rng();
	let mut vet = Vec::with_capacity(size);
	for _ in 0..size {
        	vet.push(rng.gen_range(0..100));
	}
	vet
}
fn main() {
	let inicio = Instant::now();
	let size = 10;
	let mut vet = gera_vet(size);
	println!("Vetor original: {:?}", vet);
	bubble_sort(&mut vet);
	println!("Vetor ordenado: {:?}", vet);
	let duracao = Instant::now() - inicio;
	println!("{:?}", duracao);
}

fn bubble_sort(vet: &mut Vec<i32>) {
    let n = vet.len();
    for j in 0.. n - 1 {    
        for i in 0.. n - 1 {
            if vet[i] > vet[i + 1] {
                vet.swap(i, i+1);
            }
        }
    }
    vet;
}
