use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use anyhow::Context;
use clap::{Parser, value_parser};
use seq_macro::seq;

type PartFn = fn(&str) -> String;

seq!(N in 1..=25 {
    #(mod day~N;)*

    #[used]
    static FNS: [[PartFn; 2]; 25] = [
        #(
            [day~N::part1, day~N::part2],
        )*
    ];
});

#[derive(Parser)]
#[allow(clippy::enum_variant_names)]
enum Args {
    #[clap(alias = "rp")]
    RunPart {
        day: usize,
        part: usize,
        #[arg(short, long)]
        input: Option<String>,
        #[arg(short = 't', long)]
        show_time: bool,
        #[arg(
            short = 'n', long,
            default_value_if("show_time", "true", "100"),
            default_value_t = 1,
            value_parser = value_parser!(u32).range(1..)
        )]
        iterations: u32,
    },
    #[clap(alias = "rd")]
    RunDay {
        day: usize,
        #[clap(short, long)]
        input: Option<String>,
        #[arg(short = 't', long)]
        show_time: bool,
        #[arg(short = 'T', long)]
        show_total_time: bool,
        #[arg(
            short = 'n', long,
            default_value_if("show_time", "true", "100"),
            default_value_if("show_total_time", "true", "100"),
            default_value_t = 1,
            value_parser = value_parser!(u32).range(1..)
        )]
        iterations: u32,
    },
    #[clap(alias = "ra")]
    RunAll {
        #[arg(short = 't', long)]
        show_time: bool,
        #[arg(short = 'T', long)]
        show_total_time: bool,
        #[arg(
            short = 'n', long,
            default_value_if("show_time", "true", "10"),
            default_value_if("show_total_time", "true", "10"),
            default_value_t = 1,
            value_parser = value_parser!(u32).range(1..)
        )]
        iterations: u32,
    },
}

fn run_part(
    day: usize,
    part: usize,
    input: Option<String>,
    show_time: bool,
    acc: Option<&mut Duration>,
    iterations: u32,
) -> anyhow::Result<()> {
    let input = match input {
        Some(input) => input,
        None => std::fs::read_to_string(format!("input/day{}.txt", day))
            .context("Input for this day isn't available.")?,
    };
    let f = &FNS[day - 1][part - 1];
    let now = Instant::now();
    let output = f(&input);
    for _ in 1..iterations {
        black_box(f(&input));
    }
    let elapsed = now.elapsed();
    println!("===== Day {} Part {} =====", day, part);
    println!("{}", output);
    if show_time {
        println!(
            "Average runtime across {iterations} iterations: {:.3?}",
            elapsed / iterations
        );
    }
    if let Some(acc) = acc {
        *acc += elapsed;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args {
        Args::RunPart {
            day,
            part,
            input,
            show_time,
            iterations,
        } => run_part(day, part, input, show_time, None, iterations),
        Args::RunDay {
            day,
            input,
            show_time,
            show_total_time,
            iterations,
        } => {
            let mut acc = show_total_time.then_some(Duration::ZERO);

            run_part(day, 1, input.clone(), show_time, acc.as_mut(), iterations)?;
            run_part(day, 2, input, show_time, acc.as_mut(), iterations)?;

            if let Some(acc) = acc {
                println!(
                    "Average total time across {iterations} iterations: {:.3?}",
                    acc / iterations
                );
            }
            Ok(())
        }
        Args::RunAll {
            show_time,
            show_total_time,
            iterations,
        } => {
            let mut acc = show_total_time.then_some(Duration::ZERO);
            for day in 1..=25 {
                run_part(day, 1, None, show_time, acc.as_mut(), iterations)?;
                run_part(day, 2, None, show_time, acc.as_mut(), iterations)?;
            }

            if let Some(acc) = acc {
                println!(
                    "Average time across {iterations} iterations: {:.3?}",
                    acc / iterations
                );
            }
            Ok(())
        }
    }
}
