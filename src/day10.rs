// Warning: Awful code ahead ^.^

use crate::prelude::*;

pub fn pts((inits, instrs): (Vec<Initializer>, HashMap<usize, Instruction>)) -> Result<String> {
    let mut outputs = HashMap::new();
    let mut bots = HashMap::new();

    fn write_target(
        outputs: &mut HashMap<usize, u32>,
        bots: &mut HashMap<usize, (u32, Option<u32>)>,
        target: Target,
        value: u32,
    ) {
        match target {
            Target::Bot(bot) => {
                bots.entry(bot)
                    .and_modify(|(_, v)| *v = Some(value))
                    .or_insert((value, None));
            }
            Target::Output(out) => {
                outputs.insert(out, value);
            }
        }
    }

    for init in inits {
        write_target(&mut outputs, &mut bots, Target::Bot(init.0), init.1);
    }

    let mut pt1 = None;

    loop {
        let mut bot_iter = bots.iter_mut();
        let update = loop {
            let (bot, (a, b)) = match bot_iter.next() {
                Some(v) => v,
                None => break None,
            };
            let b = match b {
                Some(b) => b,
                None => continue,
            };
            let instr = match instrs.get(&bot) {
                Some(instr) => instr,
                None => continue,
            };

            let (a, b) = (*a.min(b), *a.max(b));
            if a == 17 && b == 61 {
                pt1 = Some(*bot);
            }
            break Some((*bot, instr.low_to, a, instr.high_to, b));
        };
        if let Some((bot, low_to, a, high_to, b)) = update {
            write_target(&mut outputs, &mut bots, low_to, a);
            write_target(&mut outputs, &mut bots, high_to, b);
            bots.remove(&bot);
        } else {
            break;
        }
    }

    Ok(format!(
        "{}\n{}",
        pt1.unwrap(),
        outputs[&0] * outputs[&1] * outputs[&2]
    ))
}

pub fn parse(s: &str) -> IResult<&str, (Vec<Initializer>, HashMap<usize, Instruction>)> {
    use parsers::*;

    fn target(s: &str) -> IResult<&str, Target> {
        alt((
            map(preceded(tag("bot "), usize_str), Target::Bot),
            map(preceded(tag("output "), usize_str), Target::Output),
        ))(s)
    }

    fold_many1(
        alt((
            map(
                preceded(
                    tag("value "),
                    pair(
                        terminated(u32_str, tag(" goes to bot ")),
                        terminated(usize_str, opt(line_ending)),
                    ),
                ),
                |(value, bot)| (Some((bot, value)), None),
            ),
            map(
                tuple((
                    preceded(tag("bot "), usize_str),
                    preceded(tag(" gives low to "), target),
                    terminated(preceded(tag(" and high to "), target), opt(line_ending)),
                )),
                |(bot, low_to, high_to)| (None, Some((bot, Instruction { low_to, high_to }))),
            ),
        )),
        (Vec::new(), HashMap::new()),
        |mut acc: (Vec<_>, HashMap<_, _>), (init, instr)| {
            if let Some(init) = init {
                acc.0.push(init);
            }
            if let Some((bot, instr)) = instr {
                acc.1.insert(bot, instr);
            }
            acc
        },
    )(&s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Output(usize),
    Bot(usize),
}

type Initializer = (usize, u32);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    low_to: Target,
    high_to: Target,
}

#[test]
pub fn day10() -> Result<()> {
    const EXAMPLE: &'static str = "\
value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

    #[rustfmt::skip] let example = (
        vec![(2, 5), (1, 3), (2, 2)],
        HashMap::from_iter([
            (2, Instruction { low_to: Target::Bot(1), high_to: Target::Bot(0) }),
            (1, Instruction { low_to: Target::Output(1), high_to: Target::Bot(0) }),
            (0, Instruction { low_to: Target::Output(2), high_to: Target::Output(0) }),
        ].iter().cloned())
    );

    test_parse!(parse, EXAMPLE => example);

    Ok(())
}
