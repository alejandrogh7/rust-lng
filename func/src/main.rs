fn sum_one(arg: i32) -> i32 {
    let final_num: i32 = arg + 1;
    println!("{}", final_num);

    return final_num;
}

fn main() {
    let op: i32 = sum_one(10);
    let op2: i32 = sum_one(op);
    println!("{} and {}", op, op2);
}
