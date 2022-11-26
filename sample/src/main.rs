fn main() {

    //function

    //simply function
    print_hello();
    print_add(1u8, 2u8);
    println!("add: {}", add(1u8, 2u8));

    let v1 = vec![1; 3];
    println!("sum_vec: {}", sum_vec(v1));

    //function in struct in Vector. functional pointer?
    //better to use enum impl. maybe.
    calc_fn(Calc::ADD, 1, 2);
    calc_fn(Calc::MULTI, 2, 2);
}

fn print_hello(){
    println!("Hello")
}

fn print_add(x: u8, y: u8){
    println!("x + y = {}", x + y)
}

fn add(x: u8, y: u8) -> u8{
    return x + y
}
fn multi(x: u8, y: u8) -> u8{
    return x * y
}

fn sum_vec(v: Vec<u8>) -> u8{
    let mut sum = 0;
    for i in v{
        sum += i;
    }
    return sum
}

#[derive(PartialEq)]
enum Calc{
    ADD,
    MULTI,
}
struct CalcT{
    c:Calc,
    f: fn(u8, u8)->u8,
}
fn calc_fn(c: Calc, x:u8, y:u8){
    let calclation =
        vec![CalcT{c: Calc::ADD, f: add},
            CalcT{c: Calc::MULTI, f: multi}];

    for i in 0..calclation.len(){
        if c == calclation[i].c {
            println!("{}", (calclation[i].f)(x, y))
        }
    }
}