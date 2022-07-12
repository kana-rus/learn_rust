use std::{
    thread,
    time::Duration,
    collections::HashMap, hash::Hash,
};


fn main() {
    let user_specified_value = 10;
    let random_number = 7;
    generate_workout(
        user_specified_value, random_number
    );
}

struct Cacher1<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
} impl<T> Cacher1<T> where T: Fn(u32) -> u32 {

    fn new(calculation: T) -> Self {
        Cacher1 {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

struct _Cacher2<'a, F, K, V>
    where
        F: Fn(&K) -> V /*+ Eq*/,
        K: PartialEq + Clone,
        V: PartialEq + Clone,
        (&'static str, &'a K): Eq + Hash,
{
    func_table: HashMap<&'static str, F>,
    value_table: HashMap<(&'static str, &'a K), V>,
}

impl<'a, F, K, V> _Cacher2<'a, F, K, V>
    where
        F: Fn(&K) -> V /*+ Eq*/ + Clone,
        K: PartialEq + Clone,
        V: PartialEq + Clone,
        (&'static str, &'a K): Eq + Hash,
{

    fn _new() -> Self {
        _Cacher2 {
            func_table: HashMap::new(),
            value_table: HashMap::new(),
        }
    }

    fn _register_func(&mut self, func_name: &'static str, func: F) {
        self.func_table.insert(func_name, func);
    }

    fn _value(&mut self, func_name: &'static str, arg: &'a K) -> V {
        match self.func_table.get(func_name) {
            None => { println!("You have to use registered function!"); panic!(); }
            Some(f) => {
                let func = f;
                match self.value_table.get(&(func_name, arg)) {
                    Some(v) => v.clone(),
                    None => func(arg),
                }
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expansive_closure = Cacher1::new(|num| {
        println!("caluculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expansive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expansive_closure.value(intensity / 2)
        );
    } else {
        match random_number {
            3 => println!("break today! Remember to stay hydrated!"),
            _ => println!(
                "Today, run for {} minutes!",
                expansive_closure.value(intensity)
            ),
        }
    }
}


#[test]
fn call_cacher1_value_with_different_values() {
    let mut c = Cacher1::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    // assert_eq!(v2, 2);
    assert_ne!(v2, 2);
}

#[test]
fn call_cacher2_value_with_different_values() {
    let mut c = _Cacher2::_new();

    const SAMPLE_FUNC_NAME: &'static str = "geneate_workout";
    let sample_func = |arg: &u32| *arg;
    c._register_func(SAMPLE_FUNC_NAME, sample_func);

    let _v1 = c._value(SAMPLE_FUNC_NAME, &1);
    let v2 = c._value(SAMPLE_FUNC_NAME, &2);
    assert_eq!(v2, 2);
}