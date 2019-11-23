#![feature(
    array_value_iter,
    const_generics,
    const_generic_impls_guard,
    optin_builtin_traits,
    stmt_expr_attributes,
    trait_alias
)]
#![allow(incomplete_features)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod assembunny;
mod astar;
mod mat2;
mod parsers;
mod prelude;
#[macro_use]
mod test_helpers;
mod vec2;
mod vec3;

use anyhow::anyhow;
use aoc_proc_macro::generate_module_list;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

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
    day21[pt1, pt2]: parse,
    day22[pt1, pt2]: parse,
    day23[pt1, pt2]: parse,
    day24[pts]: parse,
    day25[pt]: parse,
);

#[derive(Clone, Copy, PartialEq, Eq)]
enum TaskState {
    Pending,
    Running,
    Done,
}

// Used on main thread
struct TaskTracker {
    module_name: &'static str,
    part_name: &'static str,
    state: TaskState,
    output: Option<Result<String, anyhow::Error>>,
}

// Used on worker thread
struct TaskWork {
    input: String,
    function: fn(&str) -> Result<String, anyhow::Error>,
}

// Sent from worker thread to main thread
enum TaskUpdate {
    WorkStarted(usize),
    WorkDone(usize, Result<String, anyhow::Error>),
}

fn pop_work(work_queue: &Mutex<Vec<Option<TaskWork>>>) -> Option<(usize, TaskWork)> {
    let mut work_queue = work_queue.lock().unwrap();
    loop {
        let len = work_queue.len();
        match work_queue.pop()? {
            Some(work) => return Some((len - 1, work)),
            None => continue,
        }
    }
}

fn main() {
    println!("\nAdvent of Code 2016\n");
    let exclusive_day = std::env::args().skip(1).next();
    let (mut task_trackers, task_work): (Vec<_>, Vec<_>) = DAY_LIST
        .iter()
        .cloned()
        .filter(|&(module_name, _)| {
            if let Some(exclusive_day) = &exclusive_day {
                exclusive_day == module_name
            } else {
                true
            }
        })
        .flat_map(|(module_name, parts)| {
            let input = std::fs::read_to_string(format!("./data/{}.txt", module_name));

            parts
                .iter()
                .cloned()
                .map(move |(part_name, function)| match &input {
                    Ok(input) => (
                        TaskTracker {
                            module_name,
                            part_name,
                            state: TaskState::Pending,
                            output: None,
                        },
                        Some(TaskWork {
                            input: input.clone(),
                            function,
                        }),
                    ),
                    Err(err) => (
                        TaskTracker {
                            module_name,
                            part_name,
                            state: TaskState::Done,
                            output: Some(Err(anyhow!(
                                "cannot read input file ./data/{}.txt ({})",
                                module_name,
                                err
                            ))),
                        },
                        None,
                    ),
                })
        })
        .unzip();

    if task_trackers.len() == 0 {
        println!("No tasks to run");
        return;
    }

    let (result_sender, result_receiver) = mpsc::channel::<TaskUpdate>();

    let work_count = task_work.iter().filter(|work| work.is_some()).count();
    let work_queue = Arc::new(Mutex::new(task_work));
    let worker_threads = (0..(num_cpus::get() - 1).max(1).min(work_count))
        .map(|_| {
            let work_queue = work_queue.clone();
            let result_sender = result_sender.clone();
            thread::spawn(move || {
                while let Some((task_index, work)) = pop_work(&work_queue) {
                    result_sender
                        .send(TaskUpdate::WorkStarted(task_index))
                        .unwrap();
                    let result = std::panic::catch_unwind(move || (work.function)(&work.input))
                        .unwrap_or_else(|err| Err(anyhow!("panic: {:?}", err)));
                    result_sender
                        .send(TaskUpdate::WorkDone(task_index, result))
                        .unwrap();
                }
            })
        })
        .collect::<Vec<_>>();

    let mut work_left = work_count;
    while work_left > 0 {
        match result_receiver.recv().unwrap() {
            TaskUpdate::WorkStarted(idx) => {
                let task = &mut task_trackers[idx];
                task.state = TaskState::Running;
                println!("{} - {} has started", task.module_name, task.part_name);
            }
            TaskUpdate::WorkDone(idx, result) => {
                let task = &mut task_trackers[idx];
                task.state = TaskState::Done;
                task.output = Some(result);
                println!(
                    "{} - {} has finished:\n{:?}",
                    task.module_name,
                    task.part_name,
                    task.output.as_ref().unwrap()
                );
                work_left -= 1;
            }
        }
    }

    for worker_thread in worker_threads {
        worker_thread.join().unwrap();
    }
}
