use std::io;

fn main() {
    let mut isloop : bool = true;
    while isloop {
        let mut temp = String::new();
        println!(
            "入力した温度を摂氏→華氏、華氏→摂氏に変換します。
    まずは温度を入力してね(入力例：32)"
        );

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line"); // 行の読み込みに失敗しました

        let temp: i32 = temp.trim().parse().expect("Please type a number!");

        let mut cf = String::new();
        println!("次は変換する温度を入力してね。華氏→摂氏にしたいならC,摂氏→華氏にしたいならF。");

        io::stdin().read_line(&mut cf).expect("Failed to read line"); // 行の読み込みに失敗しました
        let _cf = match cf.trim(){
            "C" => {
                let temp = convert_f_to_c(temp);
                println!{"摂氏{}度だよ",temp};
                isloop = false;
            },
            "F" => {
                let temp = convert_c_to_f(temp);
                println!{"華氏{}度だよ",temp};
                isloop = false;
            },
            _ => {
                println!("CかFを入力してね");
            }
        };
    }
}

fn convert_c_to_f(x: i32) -> i32 {
    x * 9 / 5 + 32
}

fn convert_f_to_c(x: i32) -> i32 {
    (x - 32) * 5 / 9
}
