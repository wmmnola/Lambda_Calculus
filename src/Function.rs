
pub struct Function{
    pub variable: String,
    pub operation: String,
}
pub trait PrintFunction{
    fn print_function(&self);
}

impl PrintFunction for Function{
    fn print_function(&self){
        println!("λ{}.{}", self.variable, self.operation);
    }
}
