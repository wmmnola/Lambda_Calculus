pub mod Function;
use std::io;
use Function::PrintFunction;



fn main() {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Fuck you you broke me");
    println!("{}",guess);
    let mut v = vec![];
    for c in guess.chars(){
        if !(c == ' ') {
            v.push(c);
        }

    }

    parse(&v, 0);
    //function_start();
}


fn parse(input:&Vec<char>, nest_level: i32) -> bool{
    let mut functions = vec![];
    let cast = input.len() as usize;
    for c in 0..cast{
        if input[c] == '('{
            let keyword: String = input[c+1..c+4].iter().cloned().collect();
            //println!("{}", keyword);
            if keyword == "lam" {
                functions.push(function_start(input, c+4));
                return true;
            }
        }
        else{
            return false;
        }
    }
    return false;
}

fn function_start(function:&Vec<char>, index: usize) -> Function::Function{
    let mut operation = vec![];

    for c in index+2..function.len() {
        operation.push(function[c]);
        println!("{}", function[c]);
    }
    parse(&operation, 2 );

    let var = String::from("x");
    let op = String::from("x");
    let mut f = Function::Function{variable: var, operation: op, nestlevel:1};
    f.print_function();
    f
}
