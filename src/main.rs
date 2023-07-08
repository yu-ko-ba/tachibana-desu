use std::io;
use std::io::Write;

fn main() {
    loop {
        let mut s = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut s)
            .expect("キー入力を取得できませんでした。");

        print!("🍓 ");
        match s.trim_end().to_owned().as_str() {
            "橘さん" => {
                println!("はい、橘です。");
                break;
            },
            "Alice" => {
                println!("（外国語の発音だと・・・名前で呼ばれるの・・・少し・・・いいかも・・・）");
                break;
            },
            &_ => println!("橘です！")
        }
    }
}
