mod bf;

use bf::{bf_machine::BfMachine, bf_parser::BfParser};

fn main() {
    let mut machine = BfMachine::default();

    let commands = BfParser::parse_compress(
        "++++++++++[>+++++++>++++++++++>+++>+<<<<-]
    >++.>+.+++++++..+++.>++.<<+++++++++++++++.
    >.+++.------.--------.>+.>.",
    )
    .unwrap();

    let _ = machine.run(&commands);
}
