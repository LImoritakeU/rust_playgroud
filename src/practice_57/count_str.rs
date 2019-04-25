use std::io::{stdin, stdout, Write};

// https://stackoverflow.com/questions/37531903/how-do-i-print-output-without-a-trailing-newline-in-rust
/*
1. 由於要讓 Input STRING 位於問句旁邊而非下一行, 要使用 stdout().flush(),
    並且由於 flush 是 std::io::Write的一部分，要確實引入。

2. 標準輸入結束會代入"\n"，要記得trim。

3. s.len() 指的是byte數量, 如果要計算真實世界使用的字數時，改用 s.chars().count()

4. 初步知道如何使用rust  的專案結構引入管理，rust 的 "mod.rs" 跟 python 的 "__init__.py" 類似。
    然而要記得把該目錄的rust file 手動引入到該檔案，才會被讀取到。

*/

pub fn count_str() {
    print!("What is the input string? ");
    stdout().flush().unwrap();

    let mut s = String::new();
    stdin().read_line(&mut s).expect("Failed to read string");
    let trim_str = s.trim_end();

    let byte_length = trim_str.len();
    let string_length = trim_str.chars().count();

    println!("The word '{}' has {} characters and {} bytes", trim_str, string_length, byte_length);
}