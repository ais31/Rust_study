


use std::collections::HashMap;
use std::cmp::Ordering;


struct Matome1 {
    mean :f32,
    median :f32,
    mode :HashMap<i32,i32>,
}

impl Matome1 {
    fn new() -> Matome1 {
        Matome1 {mean:0.0,median:0.0,mode:HashMap::new()}
    }
}

fn main() {
    
    // let v: Vec<i32> = Vec::new();
    let v: Vec<i32> = vec![3,11,1,6,9,15,9];
    println!("入力値は");
    for input in &v {
        print!("{},", input.to_string());
    }
    println!("です。");

    let matome1: Matome1 = matome_calc_1(v);

    println!("計算結果は");
    println!("平均値：{}",matome1.mean.to_string());
    println!("中央値：{}",matome1.median.to_string());
    for mode_num in matome1.mode{
        println!("最頻値：{},出現回数：{}回",mode_num.0.to_string(),mode_num.1.to_string());

    }
}   

fn matome_calc_1(mut vec:Vec<i32>) -> Matome1{
    let mut matome1 = Matome1::new();
    let mut sum: f32 = 0.0;
    //平均値を求める
    for i in &vec {
        sum += *i as f32;
    }
    matome1.mean = sum / vec.len() as f32;

    //中央値を求める
    vec.sort();
    let mid :usize = vec.len() / 2;
    if vec.len() % 2 == 0 {
        matome1.median = (vec[mid -1] as f32 + vec[mid] as f32 )/ 2.0;
    } else {
        matome1.median = vec[mid] as f32;
    }

    //最頻値を求める
    let mut map = HashMap::new();
    for num in vec {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    let mut maxmap = HashMap::new();
    let mut max = 0;
    for record in  map{
        match &record.1.cmp(&max){
            Ordering::Greater => {
                maxmap.clear();
                maxmap.insert(record.0, record.1);
                max = record.1;
            },
            Ordering::Equal => {
                maxmap.insert(record.0, record.1);

            }
            _ => (),
        }
    }
    matome1.mode = maxmap;
    matome1

}