use rust_rut::Rut;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let rut = Rut::new(&args).unwrap_or_else(|err| {
        println!("[ ERROR ] {}", err);
        process::exit(1);
    });

    let digito_verificador = rut.calcular_digito_verificador();
    println!("El digito verificador es {}", digito_verificador);
}
