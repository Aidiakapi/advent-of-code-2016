extern crate aoc_proc_macro;
extern crate anyhow;
use aoc_proc_macro::generate_module_list;

generate_module_list!(DAY_LIST;
    day01[pt1]
);

fn main() {
    let mut output = String::new();
    for (day_name, parts) in DAY_LIST {
        for (part_name, f) in *parts {
            f("test input", &mut output).unwrap();
            println!("{} - {} - {}", day_name, part_name, output);
            output.clear();
        }
    }
}
