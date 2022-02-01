struct Input {
    pos: u32,
    text: String
}

struct Block {
    block: Block
}

struct Ast {}

struct Parser {}

fn digit(input: Input) -> Option<i32>{}

fn any_char(){}

impl Applicative for Parser {
    fn run(input: Input) -> Result<(Ast, Input), String>{}
}