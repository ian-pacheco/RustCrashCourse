// Functions - Used to store blcoks of code for re use

pub fn run() {
greeting("Hello", "Jane");

// Bind fn values to variables
let get_sum = add(5,5);
println!("Sum: {}", get_sum);

// Closure
let n3: i32 = 10;
let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
println!("C sum: {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn prime_number() {
    const LIMIT: usize = 10;
    let mut numbers = Vec::with_capacity(LIMIT); // Vetor vazio pré-alocado
    let mut i = 2; // Números menores que 2 não são primos

    // Repete até conseguir 10 números
    while numbers.len() < LIMIT {
        // Verifica se i é divisível apenas por ele mesmo e 1.
        // 2..i é um iterador exclusivo, ex.: 2..5 inclui 2, 3 e 4.
        // |x| i % x != 0 é uma clausura, que recebe um inteiro (x) e retorna um booleano.
        if (2..i).all(|x| i % x != 0) {
            numbers.push(i);
        }
        i += 1;
    }

    println!("Os {} primeiros números primos são: {:?}", LIMIT, numbers);

    // Cria um iterador para o vetor, filtra os valores e gera um novo vetor.
    // Transformações podem ser combinadas em uma única iteração em collect().
    // O underscore (_) abaixo sinaliza para o compilador tentar inferir o tipo.
    let slice: Vec<_> = numbers.iter().filter(|&&x| x > 10 && x < 20).collect();
    println!("Entre 10 e 20 encontram-se: {:?}", slice);
}