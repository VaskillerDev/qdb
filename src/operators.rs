pub struct Operator {
    exp: (String,String), //expression (key,value)
    args: String,         // args tuple for function
    function: Box<dyn Fn(&dyn Fn(T) -> ())>          // function get value from exp and implement to function arguments.
}

impl Operator{
    pub fn new() -> Operator{
        Operator {exp: ("".to_string(),"".to_string()) , args: "".to_string(),function: }
    }

    pub fn print(&mut self) {
        println!("exp: {:?} args: {}",self.exp,self.args);
    }

    pub fn set_key(&mut self,exp: String) {
        self.exp.0 = exp;
    }
}

