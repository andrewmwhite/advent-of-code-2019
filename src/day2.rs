/// --- Day 2: 1202 Program Alarm ---
///
/// https://adventofcode.com/2019/day/2
///
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, UnknownOpcodeError>;

#[derive(Debug, Clone)]
pub struct UnknownOpcodeError;

impl fmt::Display for UnknownOpcodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Encountered unknown opcode.")
    }
}

impl error::Error for UnknownOpcodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

pub fn step(position: usize, program: &mut [usize], cont: bool) -> Result<&mut [usize]> {
    match program[position] {
        1 => program[program[position + 3]] = program[program[position + 1]] + program[program[position + 2]],
        2 => program[program[position + 3]] = program[program[position + 1]] * program[program[position + 2]],
        99 => return Ok(program),
        _ => return Err(UnknownOpcodeError),
    }

    return if cont { step(position + 4, program, true) } else { Ok(program) };
}

pub fn run(program: &mut [usize]) -> Result<&mut [usize]> {
    step(0, program, true)
}

pub fn search_for_inputs(output: usize, program: &[usize]) -> Option<(usize, usize)> {
    use itertools::Itertools;

    for (noun, verb) in (0usize..99usize).cartesian_product(0usize..99usize) {
        let mut input = program.to_vec();
        input[1] = noun;
        input[2] = verb;
        match run(&mut input) {
            Err(_e) => continue,
            Ok(p) => {
                if p[0] == output {
                    return Some((noun, verb));
                }
            }
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_example_program_by_steps() {
        let mut input = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let expected_after_first_step: Vec<usize> = vec![1,9,10,70,2,3,11,0,99,30,40,50];
        step(0, &mut input, false).unwrap();
        assert_eq!(input, expected_after_first_step);

        let expected_after_second_step: Vec<usize> = vec![3500,9,10,70,2,3,11,0,99,30,40,50];
        step(4, &mut input, false).unwrap();
        assert_eq!(input, expected_after_second_step);

        let expected_after_third_step: Vec<usize> = vec![3500,9,10,70,2,3,11,0,99,30,40,50];
        step(8, &mut input, false).unwrap();
        assert_eq!(input, expected_after_third_step);
    }

    #[test]
    fn it_runs_example_program() {
        let mut input = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let expected: Vec<usize> = vec![3500,9,10,70,2,3,11,0,99,30,40,50];
        run(&mut input).unwrap();
        assert_eq!(input, expected);
    }

    macro_rules! addl_examples {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, run(input).unwrap());
            }
        )*
        }
    }

    addl_examples! {
        example_1: (&mut vec![1,0,0,0,99], vec![2,0,0,0,99]),
        example_2: (&mut vec![2,3,0,3,99], vec![2,3,0,6,99]),
        example_3: (&mut vec![2,4,4,5,99,0], vec![2,4,4,5,99,9801]),
        example_4: (&mut vec![1,1,1,4,99,5,6,0,99], vec![30,1,1,4,2,5,6,0,99]),
    }

    fn get_parsed_input() -> Vec<usize> {
        use std::io::{BufRead,BufReader};
        use std::fs;

        let fs = BufReader::new(fs::File::open("data/day2.input").unwrap());
        let lines: Vec<String> = fs.lines()
                                   .filter_map(std::result::Result::ok)
                                   .collect();
        let input: Vec<usize> = lines[0].split(',')
                                      .map(|s| s.parse::<usize>())
                                      .filter_map(std::result::Result::ok)
                                      .collect();
        input
    }

    #[test]
    fn it_solves_part1() {
        let mut input = get_parsed_input();
        // ...before running the program, replace position 1 with the value 12 and replace
        // position 2 with the value 2.
        input[1] = 12;
        input[2] = 2;
        let output = run(&mut input).unwrap();
        assert_eq!(output[0], 3850704);
    }

    #[test]
    fn it_solves_part2_example() {
        let input = get_parsed_input();
        assert_eq!(search_for_inputs(3850704, &input), Some((12, 2)));
    }

    #[test]
    fn it_solves_part2() {
        let input = get_parsed_input();
        match search_for_inputs(19690720, &input) {
            Some((noun, verb)) => assert_eq!(100 * noun + verb, 6718),
            None => assert!(false),
        }
    }
}
