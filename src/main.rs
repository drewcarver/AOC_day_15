use regex::Regex;
use std::fs;

struct Box {
    letter : char,
}
impl std::fmt::Display for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.letter)
    }
}

impl std::cmp::PartialEq for Box {
    fn eq(&self, other: &Box) -> bool {
        self.letter == other.letter
    }
    fn ne(&self, other: &Box) -> bool {
        self.letter != other.letter
    }
}

impl std::fmt::Debug for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.letter)
    }
}

//implement partialeq and debug for MoveInstruction
struct MoveInstruction {
    count: usize,
    from: usize,
    to: usize 
}

impl std::fmt::Display for MoveInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{}", self.count, self.from, self.to)
    }
}

impl std::cmp::PartialEq for MoveInstruction {
    fn eq(&self, other: &MoveInstruction) -> bool {
        self.count == other.count 
            && self.from == other.from 
            && self.to == other.to
    }
    fn ne(&self, other: &MoveInstruction) -> bool {
        self.count != other.count 
            && self.from != other.from 
            && self.to != other.to
    }
}

impl std::fmt::Debug for MoveInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{}", self.count, self.from, self.to)
    }
}

fn parse_box_columns(rows: Vec<String>) -> Vec<Vec<Box>> {
    let max_length = match rows.first() {
        Some(s) => s.len(),
        None    => 0
    };

    let range = (1..max_length).step_by(4);

    let result = range
        .map(|i| 
             rows
                .iter()
                .filter_map(move |row| match row.chars().nth(i) {
                    Some(' ') => None,
                    Some(c)   => Some(c),
                    None      => None
                })
                .map(|c| Box { letter: c })
                .collect()
        )
        .collect::<Vec<Vec<Box>>>();

   result 
}

fn parse_existing_crates(input: String) -> (Vec<String>, Vec<String>) {
   let re = Regex::new(r"[\[\]]+").unwrap();

   let split_file : (Vec<_>, Vec<_>) = input
       .lines()
       .into_iter()
       .partition(|i| re.is_match(i));

   (split_file.0.iter().map(|s| s.to_string()).collect(),
    split_file.1.iter().map(|s| s.to_string()).collect())
}

fn parse_move_instructions(input: Vec<String>) 
        -> Result<Vec<MoveInstruction>, String> {
    let re = Regex::new(r"move.*").unwrap();

    input
        .iter()
        .skip_while(|line| !re.is_match(line))
        .map(|i| sscanf::sscanf!(i, "move {usize} from {usize} to {usize}")
             .map_err(|_| "Couldn't parse instructions".to_string())
             .map(|parsed_instruction_counts| MoveInstruction 
             { 
                 count: parsed_instruction_counts.0,
                 from: parsed_instruction_counts.1,
                 to: parsed_instruction_counts.2,
             })
        )
        .collect()
}

fn execute_instructions(mut box_stacks: Vec<Vec<Box>>, 
                        move_instructions: Vec<MoveInstruction>) -> Vec<Vec<Box>> {
    move_instructions
        .iter()
        .for_each(|instruction| {
            let boxes: Vec<Box> = (1..instruction.count).into_iter().filter_map(|_| box_stacks[instruction.from - 1].pop()).collect();
            boxes.into_iter().rev().for_each(|b| box_stacks[instruction.to - 1].push(b));
        });

    box_stacks
}

fn main() {
    let result = fs::read_to_string("input.txt")
        .map_err(|_| "Unable to read from file")
        .map(parse_existing_crates)
        .map(|r| {
            let box_cols = parse_box_columns(r.0);
            let instructions = parse_move_instructions(r.1);

            let result = execute_instructions(box_cols, instructions.unwrap());

            result
        });
            
    result
        .unwrap()
        .into_iter()
        .for_each(|bc| { println!("{}", bc.last().or_else(|| Some(&Box { letter: '1' })).unwrap()); })
}

// show me an example of a rust unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = "
        [F] [Q]         [Q]        
[B]     [Q] [V] [D]     [S]        
[S] [P] [T] [R] [M]     [D]        
[J] [V] [W] [M] [F]     [J]     [J]
[Z] [G] [S] [W] [N] [D] [R]     [T]
[V] [M] [B] [G] [S] [C] [T] [V] [S]
[D] [S] [L] [J] [L] [G] [G] [F] [R]
[G] [Z] [C] [H] [C] [R] [H] [P] [D]
 1   2   3   4   5   6   7   8   9 

move 3 from 5 to 2
move 3 from 8 to 4
move 7 from 7 to 3
move 14 from 3 to 9
move 8 from 4 to 1
move 1 from 7 to 5
move 2 from 6 to 4
move 4 from 5 to 7
move 1 from 3 to 6
move 3 from 4 to 3
move 1 from 4 to 1";
        let expected = vec![
"        [F] [Q]         [Q]        ",
"[B]     [Q] [V] [D]     [S]        ",
"[S] [P] [T] [R] [M]     [D]        ",
"[J] [V] [W] [M] [F]     [J]     [J]",
"[Z] [G] [S] [W] [N] [D] [R]     [T]",
"[V] [M] [B] [G] [S] [C] [T] [V] [S]",
"[D] [S] [L] [J] [L] [G] [G] [F] [R]",
"[G] [Z] [C] [H] [C] [R] [H] [P] [D]"];

        assert_eq!(expected.iter().map(|s| s.to_string()).collect::<Vec<String>>(), parse_existing_crates(test_input.to_string()).0)
    }

    #[test]
    fn it_parses_boxes() {
        let test_input = vec![
"        [F] [Q]         [Q]        ",
"[B]     [Q] [V] [D]     [S]        ",
"[S] [P] [T] [R] [M]     [D]        ",
"[J] [V] [W] [M] [F]     [J]     [J]",
"[Z] [G] [S] [W] [N] [D] [R]     [T]",
"[V] [M] [B] [G] [S] [C] [T] [V] [S]",
"[D] [S] [L] [J] [L] [G] [G] [F] [R]",
"[G] [Z] [C] [H] [C] [R] [H] [P] [D]"].iter().map(|s| s.to_string()).collect();

        // finish the expected for me
        let expected = vec![
            vec![Box { letter: 'B' }, Box { letter: 'S' }, Box { letter: 'J' }, Box { letter: 'Z' }, Box { letter: 'V' }, Box { letter: 'D' }, Box { letter: 'G' }],
            vec![Box { letter: 'P' }, Box { letter: 'V' }, Box { letter: 'G' }, Box { letter: 'M' }, Box { letter: 'S' }, Box { letter: 'Z' }],
            vec![Box { letter: 'F' }, Box { letter: 'Q' }, Box { letter: 'T' }, Box { letter: 'W' }, Box { letter: 'S' }, Box { letter: 'B' }, Box { letter: 'L' }, Box { letter: 'C' }],
            vec![Box { letter: 'Q' }, Box { letter: 'V' }, Box { letter: 'R' }, Box { letter: 'M' }, Box { letter: 'W' }, Box { letter: 'G' }, Box { letter: 'J' }, Box { letter: 'H' }],
            vec![Box { letter: 'D' }, Box { letter: 'M' }, Box { letter: 'F' }, Box { letter: 'N' }, Box { letter: 'S' }, Box { letter: 'L' }, Box { letter: 'C' }],
            vec![Box { letter: 'D' }, Box { letter: 'C' }, Box { letter: 'G' }, Box { letter: 'R' }],
            vec![Box { letter: 'Q' }, Box { letter: 'S' }, Box { letter: 'D' }, Box { letter: 'J' }, Box { letter: 'R' }, Box { letter: 'T' }, Box { letter: 'G' }, Box { letter: 'H' }],
            vec![Box { letter: 'V' }, Box { letter: 'F' }, Box { letter: 'P' }],
            vec![Box { letter: 'J' }, Box { letter: 'T' }, Box { letter: 'S' }, Box { letter: 'R' }, Box { letter: 'D' }],
        ];

        assert_eq!(expected, parse_box_columns(test_input))
    }

    #[test]
    fn it_parses_instructions() {
        let test_input = vec![
            "move 3 from 5 to 2".to_string(),
            "move 3 from 8 to 4".to_string()
        ];
        

        let expected = Ok(vec![
            MoveInstruction { count: 3, from: 5, to: 2 },
            MoveInstruction { count: 3, from: 8, to: 4 },
        ]);

        assert_eq!(expected, parse_move_instructions(test_input))
    }

}
