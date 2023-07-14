#![feature(array_try_map)]
#![feature(array_try_from_fn)]
#![feature(exact_size_is_empty)]

use std::{num::ParseIntError, process::ExitCode};

enum Error {
    TooFewArguments,
    ParseError(usize, ParseIntError),
}

impl From<(usize, ParseIntError)> for Error {
    fn from((i, err): (usize, ParseIntError)) -> Self {
        return Error::ParseError(i, err);
    }
}

fn report_usage(pname: Option<String>) -> ExitCode {
    let pname = pname.as_deref().unwrap_or("<program>");
    eprintln!("wrong usage!");
    eprintln!("usage: {pname} <x> <y> <z>");
    ExitCode::FAILURE
}

fn report_parse_failure(arg: &str, err: ParseIntError) -> ExitCode {
    eprintln!("failed to parse argument {arg}: {err}");
    ExitCode::FAILURE
}

fn main() -> ExitCode {
    let mut args = std::env::args();
    let pname = args.next();
    let parsed =
        std::array::try_from_fn(|i| args.next().map(|s| (i, s)).ok_or(Error::TooFewArguments))
            .and_then(|args| args.try_map(|(i, s)| s.parse().map_err(|err| (i, err).into())));
    if !args.is_empty() {
        return report_usage(pname);
    }
    let (x, y, z) = match parsed {
        Ok([x, y, z]) => (x, y, z),
        Err(Error::TooFewArguments) => return report_usage(pname),
        Err(Error::ParseError(1, err)) => return report_parse_failure("x", err),
        Err(Error::ParseError(2, err)) => return report_parse_failure("y", err),
        Err(Error::ParseError(3, err)) => return report_parse_failure("z", err),
        Err(_) => unreachable!(),
    };
    let mut counter = 0;
    let mut arr: Vec<Vec<Vec<u64>>> = Vec::with_capacity(x);
    counter += 1;
    for i in 0..x {
        arr.push(Vec::with_capacity(y));
        counter += 1;
        for j in 0..y {
            arr[i].push(Vec::with_capacity(z));
            counter += 1;
            for k in 0..z {
                arr[i][j].push(2u64.pow(i as u32) + 3u64.pow(j as u32) + 4u64.pow(k as u32))
            }
        }
    }
    println!("successfully created {counter} arrays");
    for (i, arr_i) in arr.iter().enumerate() {
        for (j, arr_i_j) in arr_i.iter().enumerate() {
            for (k, arr_i_j_k) in arr_i_j.iter().enumerate() {
                println!("arr[{i}][{j}][{k}] = {arr_i_j_k}");
            }
        }
    }
    ExitCode::SUCCESS
}
