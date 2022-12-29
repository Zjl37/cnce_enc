use std::env;
use std::process;

use cnce_enc::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("解析参数出错: {err}");
        process::exit(1);
    });
//     println!("=== 文件名：{} ===", config.file_path);

    if let Err(e) = cnce_enc::run(config) {
        println!("出错了: {e}");
        process::exit(1);
    }
}
