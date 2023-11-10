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
    let v: Vec<i32> = vec![1,8,3,6,5];

    let matome1: Matome1 = matome_calc_1(v);

    println!("計算結果は{:?}", matome1);
}

fn matome_calc_1(mut vec:Vec<i32>) -> Matome1{
    let mut matome1 = Matome1::new();
    let mut sum: f32 = 0.0;
    //平均値を求める
    for i in &vec {
        sum += *i as f32;
    }
    matome1.mean = sum / vec.len() as f32;
    println!("{}",matome1.mean.to_string() );

    //中央値を求める
    vec.sort();
    let mid :usize = vec.len() / 2;
    if vec.len() % 2 == 0 {
        matome1.median = (vec[mid -1] as f32 + vec[mid] as f32 )/ 2.0;
    } else {
        matome1.median = vec[mid] as f32;
    }
    println!("{}",matome1.median.to_string() );

    //最頻値を求める



    

    matome1

}