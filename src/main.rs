use std::io;

fn main() {
    println!("Adivinhe o número!");
    println!("Por favor insira seu palpite.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Falhou ao ler a linha");

    println!("Você palpitou: {guess}");
}
