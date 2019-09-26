// Algoritmo de Trabb Pardo-Knuth

// peça por 11 números para serem lidos em uma sequência A
// inverta a sequência A
// para cada item na sequência A
//    faça uma operação
//    se o resultado ultrapassar o limite
//       alertar usuário
//    senão
//       imprimir resultado

pub fn run() {
    let f = |t: f64| t.abs().sqrt() + 5.0 * t.powi(3);
    let mut a = [0f64; 11];

    for t in &mut a {
        let mut num = String::new();
        std::io::stdin().read_line(&mut num).ok();
        *t = num.trim().parse().unwrap();
    }

    for t in a.iter().rev() {
        match f(*t) {
            y if y > 400.0 || y.is_nan() => println!("Valor muito grande"),
            y => println!("{}", y),
        }
    }
}
