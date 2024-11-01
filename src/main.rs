use rand::Rng;
use std::fmt;
use std::time::Instant;
use tabled::{Tabled, Table};

const MAX_WEIGHT: u16 = 1000;

#[derive(Clone)]
struct Item(u8, u16);
// Item(weight, value)

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

struct Solution(u64, u16, u16);
//Solution(Binary, weight, value)

#[derive(Tabled)]
struct Runtime {
    n: u8, 
    set_1: f64,
    set_2: f64,
    set_3: f64,
    average: f64,
}
//For displayying table

fn build_runtime(n: u8, set_1: f64, set_2: f64, set_3: f64) -> Runtime {
    Runtime {
        n, 
        set_1, 
        set_2, 
        set_3,
        average: (set_1 + set_2 + set_3)/3.0
    }
}


fn display_items(items: &[Item]){
    print!("[");
    for item in items.iter(){
        print!("{}, ", item);
    }
    print!("]");
}

fn gen_set(n: u8) -> Vec<Item>{
    let mut vec: Vec<Item> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let weight: u8 = rng.gen_range(50..=100);
        let value: u16 = rng.gen_range(100..=500);
        vec.push(Item(weight, value));
    };
    vec
}


fn gen_subset(items: &[Item]) {
    let size = items.len();
    let mut temp = vec![Item(0, 0); size];
    let mut start = 0u64;
    let n:u64 = 1 << size;
    print!("{:04b}  ", start);
    display_items(&temp);
    println!(" ");
    for i in 1u64..n {
        let lsb: usize = i.trailing_zeros() as usize;
        start ^= 1 << lsb;
        print!("{:04b}  ", start);
        if (start & (1u64 << lsb)) != 0{
            temp[lsb] = items[lsb].clone();
        } else {
            temp[lsb] = Item(0, 0);
        }
        display_items(&temp);
        println!(" ");
    }
}

fn gen_weight_value(items: &[Item]) {
    let mut weight: u16 = 0;
    let mut value: u16 = 0;
    let size = items.len();
    let mut start = 0u64;
    let n:u64 = 1 << size;
    for i in 1u64..n {
        let lsb: usize = i.trailing_zeros() as usize;
        start ^= 1 << lsb;
        if (start & (1u64 << lsb)) != 0{
            weight += items[lsb].0 as u16;
            value += items[lsb].1;
        } else {
            weight -= items[lsb].0 as u16;
            value -= items[lsb].1;
        }
        println!("{:04b} weight: {}, value: {}", start, weight, value)
    } 
}

fn solve_knapsack(items: &[Item]) -> Solution {
    let mut subset = 0u64;
    let mut max_weight: u16 = 0;
    let mut max_value: u16 = 0;
    let mut weight: u16 = 0;
    let mut value: u16 = 0;
    let size = items.len();
    let mut start = 0u64;
    let n:u64 = 1 << size;
    for i in 1u64..n {
        let lsb: usize = i.trailing_zeros() as usize;
        start ^= 1 << lsb;
        if (start & (1u64 << lsb)) != 0{
            weight += items[lsb].0 as u16;
            value += items[lsb].1;
        } else {
            weight -= items[lsb].0 as u16;
            value -= items[lsb].1;
        }
        if weight <= MAX_WEIGHT {
            if value > max_value {
                subset = start;
                max_weight = weight;
                max_value = value;
            }
        } else {
            continue
        }
    }
    Solution(subset, max_weight, max_value)

}


fn gen_small_cases(n: u8){
    println!("Generating random sets of size {}", n);
    let set_1 = gen_set(n);
    let set_2 = gen_set(n);
    let set_3 = gen_set(n);
    println!("Set 1 of size {}:", n);
    display_items(&set_1);
    println!("\n");
    println!("Set 2 of size {}:", n);
    display_items(&set_2);
    println!("\n");
    println!("Set 3 of size {}:", n);
    display_items(&set_3);
    println!("\n");

    println!("Generating subsets of Set 1: ");
    gen_subset(&set_1);
    println!(" ");
    println!("Generating subsets of Set 2: ");
    gen_subset(&set_2);
    println!(" ");
    println!("Generating subsets of Set 3: ");
    gen_subset(&set_3);
    println!(" ");

    println!("Generating cumulative weights and values of each subset for set 1: ");
    gen_weight_value(&set_1);
    let mut sol = solve_knapsack(&set_1);
    println!("Solution:[set: {:04b}, weight: {}, value: {}]", sol.0, sol.1, sol.2);
    println!(" ");
    println!("Generating cumulative weights and values of each subset for set 2: ");
    gen_weight_value(&set_2);
    sol = solve_knapsack(&set_2);
    println!("Solution:[set: {:04b}, weight: {}, value: {}]", sol.0, sol.1, sol.2);
    println!(" ");
    println!("Generating cumulative weights and values of each subset for set 3: ");
    gen_weight_value(&set_3);
    sol = solve_knapsack(&set_3);
    println!("Solution:[set: {:04b}, weight: {}, value: {}]", sol.0, sol.1, sol.2);
    println!(" ");
}

fn gen_actual_case(n: u8) -> Runtime {
    println!("Test case n = {}", n);
    let mut runtimes = Vec::new();
    for i in 1..=3 {
        println!("Set {}", i);
        let set = gen_set(n);
        display_items(&set);
        let start = Instant::now();
        let sol = solve_knapsack(&set);
        let runtime = start.elapsed().as_secs_f64();
        println!(" ");
        println!("Solution:[set: {:050b}, weight: {}, value: {}]", sol.0, sol.1, sol.2);
        println!("Runtime: {}", runtime);
        println!(" ");
        runtimes.push(runtime);
    }
    build_runtime(n, runtimes[0], runtimes[1], runtimes[2])
}

fn main() {
    for i in 2..=4{
        gen_small_cases(i);
    }

    let mut runtimes: Vec<Runtime> = Vec::new();

    for i in 10..=30 {
        let runtime = gen_actual_case(i);
        runtimes.push(runtime);
    }
    let table = Table::new(runtimes).to_string();
    println!("{}", table);
}
