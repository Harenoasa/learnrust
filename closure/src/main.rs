use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where T: Fn(u32) -> u32 {
    calculation: T,
    values: HashMap<u32,u32>,
}

impl<T> Cacher<T>
where T: Fn(u32)  -> u32,
{
    fn new (calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(&v)=> v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg,v);
                v
            }
        }
    }
}

fn main() {
    let tuples: Vec<(u32,u32)>= vec!((10,7),(3,6),(26,9),(4,6),(7,2));
    for tuple in tuples {
        generate_workout(tuple.0,tuple.1);
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut calculation_cache = Cacher::new(|intensity| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity * 7
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", calculation_cache.value(intensity));
        println!("Next, do {} situps!", calculation_cache.value(intensity))
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", calculation_cache.value(intensity));
            println!("Today, run for {} minutes!", calculation_cache.value(intensity));
        };
    }
}
