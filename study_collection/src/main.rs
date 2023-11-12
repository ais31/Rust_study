


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
    
    // 入力値定義
    let v: Vec<i32> = vec![3,11,1,6,9,15,9];
    println!("入力値は");
    for input in &v {
        print!("{},", input);
    }
    println!("です。");

    //平均値と中央値と最頻値を計算する
    let matome1: Matome1 = matome_calc_1(v);

    //計算結果を出力
    println!("計算結果は");
    println!("平均値：{}",matome1.mean);
    println!("中央値：{}",matome1.median);
    for mode_num in matome1.mode{
        println!("最頻値：{},出現回数：{}回",mode_num.0,mode_num.1);

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
    //昇順にソートしてベクターの中央の値を求める。
    let mid :usize = vec.len() / 2;
    if vec.len() % 2 == 0 {
        matome1.median = (vec[mid -1] as f32 + vec[mid] as f32 )/ 2.0;
    } else { // 要素が複数ある場合は2つの値の平均値
        matome1.median = vec[mid] as f32;
    }

    //最頻値を求める
    //最頻値求める用のmap(入力値：出現回数)を作成
    let mut map = HashMap::new();
    for num in vec {
        let count = map.entry(num).or_insert(0);
        //or_insertであったら追加、無かったら何もしない。でcountアップ。
        *count += 1;
    }
    let mut maxmap = HashMap::new();
    let mut max = 0;
    //要素を取り出して出現回数の最大を取ってくる。最大が複数あったらその数分コンソールに出力させるように
    for record in  map{
        match record.1.cmp(&max){
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