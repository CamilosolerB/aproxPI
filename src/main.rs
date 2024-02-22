use std::io;
use std::f64::consts;

fn main() {
    println!("Escoja el metodo por el que quiera resolver \n 1. Circunferencia completa \n 2. Cuarto de circunferencia");
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
    if number == 1 {
        primera_function()
    } else{
        segunda_funcion()
    }
}

fn exp_function(base: f64, exp: f64) -> f64{
    return exp * base.exp();
}

fn input_value() -> f64 {
    println!("Indique el valor para su aproximacion a PI");
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
    let number = input_value();
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
    let number = input_value();

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

