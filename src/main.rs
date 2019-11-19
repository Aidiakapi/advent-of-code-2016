#![feature(
    trait_alias,
    stmt_expr_attributes,
    const_generics,
    const_generic_impls_guard
)]
#![allow(incomplete_features)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod astar;
mod mat2;
mod parsers;
mod prelude;
#[macro_use]
mod test_helpers;
mod vec2;
mod vec3;

use aoc_proc_macro::generate_module_list;
use colored::Colorize;
use std::fmt::Debug;

generate_module_list!(DAY_LIST;
    day01[pt1, pt2]: parse,
    day02[pt1, pt2]: parse,
    day03[pt1, pt2]: parse,
    day04[pt1, pt2]: parse,
    day05[pt1, pt2],
    day06[pt1, pt2]: parse,
    day07[pt1, pt2]: parse,
    day08[pts]: parse,
    day09[pt1, pt2],
    day10[pts]: parse,
    day11[pt1, pt2]: parse,
    day12[pt1, pt2]: parse,
    day13[pt1, pt2]: parse,
    day14[pt1, pt2],
    day15[pt1, pt2]: parse,
    day16[pt1, pt2]: parse,
    day17[pt1, pt2]: parse,
    day18[pt1, pt2]: parse,
    day19[pt1, pt2]: parse,
    day20[pt1, pt2]: parse,
);

fn main() {
    if cfg!(windows) {
        colored::control::set_virtual_terminal(true).unwrap();
    }

    println!(
        "\n{} {} {} {}\n",
        "Advent".bright_red().bold(),
        "of".bright_white(),
        "Code".bright_green().bold(),
        "2016".bright_blue()
    );

    let exclusive_day = {
        let mut args = std::env::args();
        args.next();
        args.next()
    };

    for (day_name, parts) in DAY_LIST {
        if let Some(exclusive_day) = &exclusive_day {
            if exclusive_day != day_name {
                continue;
            }
        }

        let input: String = match std::fs::read_to_string(format!("./data/{}.txt", day_name)) {
            Ok(value) => value,
            Err(err) => {
                println!(
                    "{} {} ({})\n",
                    day_name.green(),
                    "error: cannot read day input".red().bold(),
                    err
                );
                continue;
            }
        };
        let input = input.trim();

        for (part_name, part_func) in *parts {
            println!("{} {}", day_name.green(), part_name.blue().bold());
            match std::panic::catch_unwind(|| match part_func(&input) {
                Ok(output) => println!("{}", output.bright_white()),
                Err(err) => println!(
                    "{} {}",
                    "error".underline().bright_red(),
                    format!("{:?}", err).red()
                ),
            }) {
                Ok(()) => {}
                Err(err) => {
                    if let Some(s) = err.downcast_ref::<&str>() {
                        println!(
                            "{} {}",
                            "panic".underline().bright_red(),
                            format!("{}", s).red()
                        );
                    } else if let Some(d) = err.downcast_ref::<&dyn Debug>() {
                        println!(
                            "{} {}",
                            "panic".underline().bright_red(),
                            format!("{:?}", d).red()
                        );
                    } else {
                        println!("{}", "panic without message".underline().bright_red());
                    }
                }
            };
        }
        println!();
    }
}
