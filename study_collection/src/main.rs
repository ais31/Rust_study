struct Matome1 {
    mean :i64,
    median :i64,
    mode :i64,
}

impl Matome1 {
    fn new() -> Matome1 {
        Matome1 {mean:0,median:0,mode:0}
    }
}

fn main() {
    
    // let v: Vec<i32> = Vec::new();
    let v = vec![1,2,3,4];

    let matome1 = matome_calc_1(v);

    //println!("計算結果は{:?}", matome1);
}

fn matome_calc_1(vec:Vec<i32>) -> Matome1{
    let mut matome1 = Matome1::new();
    let mut sum = 0;
    for i in &vec {
        sum += i;
    }
    matome1.mean = sum / vec.len();
    println!(matome1.mean );

}