use std::env;
use std::process;

extern crate cnce_enc;
use cnce_enc::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("解析参数出错: {err}");
        process::exit(1);
    });
//     println!("=== 文件名：{} ===", config.file_path);

    if let Err(e) = cnce_enc::run(config) {
        eprintln!("出错了: {e}");
        process::exit(1);
    }
}
