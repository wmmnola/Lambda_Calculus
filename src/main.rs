pub mod Function;
use std::io;
use Function::PrintFunction;


fn main() {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Fuck you you broke me");
    println!("{}",guess);
    let mut v = vec![];
    for c in guess.chars(){
        v.push(c);
    }

    //parse(&v);
    //function_start();
}


fn parse(input:&Vec<char>){

    let cast = input.len() as usize;
    for c in 0..cast{
        if input[c] == '('{
            let keyword: String = input[c+1..c+4].iter().cloned().collect();
            println!("{}", keyword);
            if(keyword == "lam" ){
                function_start(input[c..]);
            }
        }
    }
}

fn function_start(function:Vec<char>) -> Function::Function{
    let var = String::from("x");
    let op = String::from("x");
    let mut f = Function::Function{variable: var, operation: op};
    f.print_function();
    f
}
