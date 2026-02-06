//! 仓位管理计算器
//! f* = (bp - q) / b
//! 其中 b 为赔率-1，p 为胜率，q = 1-p

mod app;
mod arbitrage;
mod cli;
mod display;
mod interactive;
mod kelly;
mod nash;
mod portfolio;
mod portfolio_input;
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
                } else if args[1] == "-n" {
                    interactive::interactive_nash();
                } else if args[1] == "-K" {
                    interactive::interactive_portfolio_correlated();
                } else if args[1] == "-k" {
                    interactive::interactive_portfolio();
                }
            }
            _ => unreachable!(),
        }
    } else {
        cli::handle_args(args);
    }
}
