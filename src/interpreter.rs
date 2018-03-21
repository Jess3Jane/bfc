use super::parser::ASTNode;
use std::io::{Read, Error, stdin};

struct ExecutionState {
    memory: Vec<u8>,
    dp: usize,
}

impl ASTNode {
    pub fn run(&mut self) -> Result<(), Error>{
        let mut es = ExecutionState { memory: vec![0], dp: 0 };
        self.run_part(&mut es)
    }

    fn run_part(&self, es: &mut ExecutionState) -> Result<(), Error> {
        match self {
            &ASTNode::None => {},
            &ASTNode::PUTC => { 
                print!("{}", es.memory[es.dp] as char); 
            },
            &ASTNode::GETC => { 
                let mut buff = [0; 1];
                stdin().read_exact(&mut buff)?;
                es.memory[es.dp] = buff[0]; 
            },
            &ASTNode::DP(val) => {
                es.dp = (es.dp as isize + val) as usize;
                if es.dp < 0 {
                    es.dp = 0;
                } else if es.dp >= es.memory.len() {
                    for _ in 0..es.dp - es.memory.len() + 1 {
                        es.memory.push(0);
                    }
                }
            },
            &ASTNode::ADD(val) => {
                es.memory[es.dp] = (val + es.memory[es.dp] as i8) as u8;
            },
            &ASTNode::Body(box ref x) => {
                for op in x {
                    op.run_part(es);
                }
            },
            &ASTNode::Loop(box ref x) => {
                loop {
                    if es.memory[es.dp] != 0 {
                        x.run_part(es);
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}
