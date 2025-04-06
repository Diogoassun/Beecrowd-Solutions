use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    
    io::stdin().read_line(&mut a).expect("Erro ao ler o valor de a");


    io::stdin().read_line(&mut b).expect("Erro ao ler o valor de b");

    let a: i32 = a.trim().parse().expect("Digite um número válido para a");
    let b: i32 = b.trim().parse().expect("Digite um número válido para b");

    let res = a + b;

    println!("X = {}", res);
}
