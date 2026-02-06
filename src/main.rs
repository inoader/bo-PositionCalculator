//! 凯利公式计算器
//! f* = (bp - q) / b
//! 其中 b 为赔率-1，p 为胜率，q = 1-p

mod arbitrage;
mod cli;
mod display;
mod interactive;
mod kelly;
mod types;
mod validation;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 检查是否为交互式模式调用
    if cli::is_interactive_call(&args) {
        match args.len() {
            1 => interactive::interactive(),
            2 => {
                if args[1] == "-p" {
                    interactive::interactive_polymarket();
                } else if args[1] == "-s" {
                    interactive::interactive_stock();
                } else if args[1] == "-a" {
                    interactive::interactive_arbitrage();
                } else if args[1] == "-A" {
                    interactive::interactive_multi_arbitrage();
                }
            }
            _ => unreachable!(),
        }
    } else {
        cli::handle_args(args);
    }
}
