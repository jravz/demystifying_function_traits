

pub trait Caller<T,V> 
{
    fn call(&self,args:T) -> V;
}

pub trait FromInputParam<T> {
    fn from_input_param(input:&T) -> Self;
}


macro_rules! dynamic_function ({$InputType:ident; $OutputTrait:ident; $($param:ident)* } => {
    #[allow(dead_code)]
    impl<Func,V,$($param,)*> Caller<($($param,)*),V> for Func
    where Func: Fn($($param),*) -> V, 
    V: $OutputTrait {
        
        #[inline]
        #[allow(non_snake_case)]
        fn call(&self, ($($param,)*): ($($param,)*)) -> V {
            (self)($($param,)*)
        }
    }
    
    #[allow(dead_code)]
    impl<$($param:FromInputParam<$InputType>),*> FromInputParam<$InputType> for ($($param,)*) {
        fn from_input_param(input:&$InputType) -> ($($param,)*) {
            ($(<$param as FromInputParam<$InputType>>::from_input_param(input),)*)                
        }
    }
});