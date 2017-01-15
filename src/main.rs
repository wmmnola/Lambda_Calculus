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


fn parse(input:&Vec<char>, nest_level: i32) -> Function::Function{
    let mut nested_functions = vec![];
    let mut bound = Function::FunctionBound{start: 0, stop: 0, nest_level: nest_level};
    let mut f = Function::Function{bound: Function
                                   ::FunctionBound{start: 0, stop: 0, nest_level: nest_level},
                                   nested_functions: vec![]};
    let cast = input.len() as usize;
    for c in 0..cast{
        println!("{}", input[c]);
        if input[c] == '('{
            println!("Function Start at nest level {}", c);
            bound.start = c;
            let out = input[c+1..].to_vec();
            nested_functions.push(parse(&out, nest_level+1));
            break;
        }
        if input[c] == ')'{
            bound.stop = c;
            println!("Function Stop at nest level {}", c);
        }
    }
    f.bound = bound;
    f.nested_functions = nested_functions;
    f
}

// fn function_start(function:&Vec<char>, index: usize) -> Function::Function{
//     let mut operation = vec![];
//
//     for c in index+2..function.len() {
//         operation.push(function[c]);
//         println!("{}", function[c]);
//     }
//     parse(&operation, 2 );
//
//     let var = String::from("x");
//     let op = String::from("x");
//     let mut f = Function::Function{variable: var, operation: op, nestlevel:1};
//     f.print_function();
//     f
// }
