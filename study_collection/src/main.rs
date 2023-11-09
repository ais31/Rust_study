#[derive(Debug)]
struct Matome1 {
    mean :f32,
    median :f32,
    mode :f32,
}

impl Matome1 {
    fn new() -> Matome1 {
        Matome1 {mean:0.0,median:0.0,mode:0.0}
    }
}

fn main() {
    
    // let v: Vec<i32> = Vec::new();
    let v = vec![1,2,3,4];

    let matome1 = matome_calc_1(v);

    println!("計算結果は{:?}", matome1);
}

fn matome_calc_1(vec:Vec<i32>) -> Matome1{
    let mut matome1 = Matome1::new();
    let mut sum = 0.0;
    for i in &vec {
        sum += *i as f32;
    }
    matome1.mean = sum / vec.len() as f32;
    println!("{}",matome1.mean.to_string() );

    matome1

}