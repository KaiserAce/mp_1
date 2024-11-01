use rand::Rng;
use std::fmt;

const MAX_WEIGHT: u16 = 1000;

#[derive(Clone)]
struct Item(u8, u16);

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

struct Solution(u64, u16, u16);

fn display_items(items: &Vec<Item>){
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


fn gen_subset(items: &Vec<Item>) {
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

fn gen_weight_value(items: &Vec<Item>) {
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

fn solve_knapsack(items: &Vec<Item>) -> Solution{
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
        if max_weight <= MAX_WEIGHT {
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
    solve_knapsack(&set_1);
    println!(" ");
    println!("Generating cumulative weights and values of each subset for set 2: ");
    gen_weight_value(&set_2);
    solve_knapsack(&set_2);
    println!(" ");
    println!("Generating cumulative weights and values of each subset for set 3: ");
    gen_weight_value(&set_3);
    solve_knapsack(&set_3);
    println!(" ");
}

fn gen_actual_cases() {

}

fn print_tabled_results() {

}

fn main() {
    for i in 2..=4{
        gen_small_cases(i);
    }
    for i in 10..=40 {

    }
}
