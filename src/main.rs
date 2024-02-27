use std::io;
use std::f64::consts;

fn main() {
    println!("Escoja el metodo por el que quiera resolver \n 1. Circunferencia completa \n 2. Cuarto de circunferencia \n 3. Funcion normal");
    let mut resolver = String::new();
    io::stdin().read_line(&mut resolver).expect("Error reading");
    let input = resolver.trim();
    let number: i32 = match input.parse() {
        Ok(num) => num,
        Err(_)=>{
            println!("Numero no valido");
            return;
        }
    };
    match  number{
        1=> primera_function(),
        2=> segunda_funcion(),
        3=> tercera_funcion(),
        _=>println!("Indique una opcion valida")
    }
}

fn exp_function(base: f64, exp: f64) -> f64{
    return exp * base.exp();
}

fn input_value(message: &str) -> f64 {
    println!("{}",message);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    let input = input.trim();
    let number: f64 = match input.parse() {
        Ok(num) => num,
        Err(_)=>{
            println!("Numero no valido");
            return 0.0;
        }
    };
    return number
}

fn primera_function(){
    let mut exp: f64;
    let mut sen: f64;
    let mut value: f64;
    let pi = consts::PI;
    let number = input_value("Indique el valor para su aproximacion a PI");
    for _i in 1..=number as i32{
        exp = exp_function(2.0,_i as f64);
        sen = pi / exp;
        value = exp * sen.sin();
        println!("El valor de Pi con respecto a {} es: {}", _i, value);
    }
}

fn segunda_funcion() {
    let mut exp: f64;
    let mut value: f64;
    let mut frac: f64;
    let mut sum: f64 = 0.0; // Use "sum" instead of "quat" for clarity
    let number = input_value("Indique el valor para su aproximacion a PI");

    for n in 1..=number as i32 {
        exp = 2.0f64.powf(n as f64 - 1.0); // Calculate 2^(n-1) directly
        frac = 1.0 / (exp * exp); // Calculate (1/2^(n-1))^2 directly
        if exp == 1.0 {
            sum = 1.0;
        }
        for i in 1..=exp as i32 {  // Iterate up to 2^(n-1) inclusive
            sum += (exp.powf(2.0) - (i as f64).powf(2.0)).sqrt(); // Calculate and accumulate the square root
        }
        value = 4.0 * frac * sum; // Apply the formula for pi
        println!("El valor aproximado de pi con {} repeticiones es {}", n, value);
    }
}
fn tercera_funcion(){
    const E: f64 = 2.71828182845904523536028747135266250_f64;
    let b = input_value("Indique el valor del limite superior de la integral");
    let n = input_value("Indique el numero de veces de partir la funcion en rectangulos");
    let division;
    let mut exp: f64;
    let mut sum: f64 = 0.0;
    let result: f64;
    division = b / n;
    for i in 1..=n as i32 {
        exp = -1.0*(division * (i as f64 + 1.0)).powf(2.0) / 2.0;
        exp = E.powf(exp);
        sum += exp;
    }
    result = 2.0 * (division * sum).powf(2.0);
    println!("La aproximacion con {} rectangulos y de limite {} es: {}",n, b, result);
}

