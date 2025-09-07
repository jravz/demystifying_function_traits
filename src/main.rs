#[macro_use]
mod macros;
use macros::*;


// Setting up Output type and implementing the trait for the same
pub trait Responder {
    fn value(&self) -> String;
}


struct Response(String);

impl Responder for Response {
    fn value(&self) -> String {
        self.0.clone()
    }
}

struct First(String);

impl First {
    fn value(&self) -> String {
        self.0.to_string()
    }
}

struct Second(String,String);

impl Second {
    fn extract(&self) -> String {
        format!("|{} - {}|",self.0.to_string(),self.1.to_string())
    }
}
// ### SETTING UP THE FUNCTION HANDLERS WITH TRAIT IMPLEMENTATIONS ###
// Setting up the functions for multiple parameters
dynamic_function! {Request; Responder; A}
dynamic_function! {Request; Responder; A B}
dynamic_function! {Request; Responder; A B C}
dynamic_function! {Request; Responder; A B C D}
dynamic_function! {Request; Responder; A B C D E}


// Declaring the functions themselves to be used as function handlers
fn first(arg:First) -> Response {
    Response(arg.value())
}

fn second(arg:First, arg2:Second) -> Response {
    Response(format!("{} {}",arg.value(),arg2.extract()))
}

fn third(arg:First,arg2:Second, arg3:First) -> Response {
    Response(format!("{} {} {}",arg.value(),arg2.extract(),arg3.value()))
}

fn third_2(arg:First,arg2:First, arg3:First) -> Response {
    Response(format!("{} {} {}",arg.value(),arg2.value(),arg3.value()))
}

//  ### REQUEST TYPE ### 
// Setting up the input for extracting values for the function params
#[derive(Clone)]
pub struct Request(String);

impl Request {
    fn respond<T,Args:FromInputParam<Request>,V:Responder>(self, func:T) -> V
    where T: Caller<Args,V>    
    {    
        let args = Args::from_input_param(&self);
        func.call(args)
    }
}

// ## SETUP FUNCTION PARAMS WITH FromInputParam IMPLEMENTATION FOR EXTRACTION ###
// Implement FromInputParam for the function parameter types
impl FromInputParam<Request> for First {
    fn from_input_param(input:&Request) -> First {
        First(input.0.clone())
    }
}

impl FromInputParam<Request> for Second {
    fn from_input_param(input:&Request) -> Second {
        Second(input.0.clone(),input.0.clone())
    }
}


fn main() {
    let request: Request = Request("Value".to_string());    
    let output: Response = request.respond(first);
    println!("One: {}",output.0);

    let request: Request = Request("Value".to_string());   
    let output: Response = request.respond(second);
    println!("Two: {}",output.0);

    let request: Request = Request("Value".to_string());  
    let output: Response = request.respond(third);
    println!("Three: {}",output.0);

    let request = Request("Value".to_string());      
    let output = request.respond(third_2);
    println!("Three Other: {}",output.0);
    

}