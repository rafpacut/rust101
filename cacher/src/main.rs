use std::collections::HashMap;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    valuesMap: HashMap<u32,u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T>
    {
        Cacher {
            calculation,
            valuesMap: HashMap::new(),
        }
    }

    fn get_value(&mut self, arg: u32) -> u32 {
        match self.valuesMap.get(&arg) {
            Some(&v) => v,
            None => {
                    let v = (self.calculation)(arg);
                    self.valuesMap.insert(arg,v);
                    v
            }
        }
        //match self.value {
            //Some(v) => v,
            //None => {
                //let v = (self.calculation)(arg);
                //self.value = Some(v);
                //v
            //}
        //}
    }
}


#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.get_value(1);
    let v2 = c.get_value(2);

    println!("{}",v1);
    println!("{}",v2);

    assert_eq!(v2, 2);
}

fn main() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.get_value(1);
    let v2 = c.get_value(2);

    println!("{}",v1);
    println!("{}",v2);
}