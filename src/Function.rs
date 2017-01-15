
pub struct Function{
    pub bound: FunctionBound, //Gives the function location within the string
    pub nested_functions: Vec<Function>, //Feedback Loop?
}
pub trait PrintFunction{
    fn print_function(&self);
}

impl PrintFunction for Function{
    fn print_function(&self){
        //println!("λ{}.{}", self.variable, self.operation);
    }
}

pub struct Variable{
    pub variable: String,
}

pub struct FunctionBound{
    pub start: usize,
    pub stop: usize,
    pub nest_level: i32,
}
