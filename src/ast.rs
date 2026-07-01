use std::collections::HashMap;

pub type Env = HashMap<String,664>;

pub trait Node {
    fn eval(&self,env:&mut Env) -> Result<f64,String>;
}
//------Number litreal-------
pub struct Num {
    pub value:f64,
}

 impl Node for Num {
    fn eval(&self,_env: &mut Env) -> Result<f64,String>{
        Ok(self.value);
    }
 }
 //------- Binary Operations-------
 pub struct BinOp{
    pub left : Box<dyn Node>,
    pub op : char,
    pub right: Box<dyn Node>,
 }
 impl Node for BinOp {
    fn eval(&self,env:&mut Env) -> Result<f64,String>{
        let l = self.left.eval(env)?;
        let r = self.right.eval(env)?;
        match self.op {
            '+' => Ok(l+r),
            '-' => Ok(l-r),
            '*' => Ok(l*r),
            '/' => {
                if r == 0.0 {
                    Err("division by zero".to_string())
                }else {
                    Ok(l/r)
                }
            }
            '^' => Ok(l.powf(r)),
            _ => Err(format!("unknown operator: {}",self.op)),
        }
    }
    //------ variable loop------
    pub struct Var {
        pub name:String,
    }
    impl Node for Var {
        fn eval(&self,env:&mut Env) -> Result<f64,String>{
            env.get(&self.name)
            .copied()
            .ok_or(format!("undefined variable {}",self.name))
        }
    }
    //-------variable assignment----------
    pub struct Assignment {
        pub name:String,
        pub value:Box<dyn Node>,
    }
    impl Node for Assignment {
        fn eval(&self,env:&mut Env) ->Result<f64,String>{
            let val = self.value.eval(env)?;
            env.insert(self.name.clone(),val);
            Ok(val)
        }  
    }
 }

