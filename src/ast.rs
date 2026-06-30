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
 }

