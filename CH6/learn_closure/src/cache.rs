//实现一个缓存，只处理第一次传入的值
pub struct Cache<T> 
    where T:Fn(u32) -> u32
{
    calculation: T,
    value :Option<u32>,
}
impl<T> Cache<T>
    where T:Fn(u32) -> u32
{
    pub fn new(calculation:T) -> Cache<T>{
        Cache{
            calculation,
            value:None,
        }
    }
    pub fn value(&mut self,arg:u32) -> u32{
        match self.value{
            Some(v)=>v,
            None=>{
                let v = (self.calculation)(arg);
                //这里为什么要加一个括号呢？因为self.calculation是一个函数指针，
                //而后面arg的括号是闭包传参数带的括号
                //to call the function stored in `calculation`, surround the field access with parentheses: `(`, `)`
                self.value = Some(v);
                v
            }
        }
    }
}



