use std::str::Chars;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ASTNode {
    None,
    PUTC,
    GETC,
    DP(isize),
    ADD(i8),
    Body(Box<Vec<ASTNode>>),
    Loop(Box<ASTNode>),
}

pub fn parse(program: &str) -> Result<ASTNode, &'static str> {
    if program.len() < 1 {
        return Err("can't parse an empty program")
    }

    let mut tokenizer = Tokenizer::new(program);
    let mut body = Vec::new();

    while tokenizer.token != None {
        body.push(parse_token(&mut tokenizer)?);
    }

    Ok(ASTNode::Body(Box::new(body)))
}

fn parse_token(tokenizer: &mut Tokenizer) -> Result<ASTNode, &'static str> {
    match tokenizer.token {
        Some(c) if c == '-' || c == '+' => parse_add(tokenizer),
        Some(c) if c == '>' || c == '<' => parse_dp(tokenizer),
        Some('[') => parse_loop(tokenizer),
        Some(']') => Err("Unmatched ]"),
        Some(',') => {
            tokenizer.consume();
            Ok(ASTNode::GETC)
            },
        Some('.') => {
            tokenizer.consume();
            Ok(ASTNode::PUTC)
            },
        Some(_) => Err("The tokenizer is broken"),
        None => Ok(ASTNode::None),
    }
}

fn parse_add(tokenizer: &mut Tokenizer) -> Result<ASTNode, &'static str> {
    let mut count = 0;
    loop {
        match tokenizer.token {
            Some('+') => count += 1,
            Some('-') => count -= 1,
            _ => return Ok(ASTNode::ADD(count)),
        };
        tokenizer.consume();
    }
}

fn parse_dp(tokenizer: &mut Tokenizer) -> Result<ASTNode, &'static str> {
    let mut count = 0;
    loop {
        match tokenizer.token {
            Some('>') => count += 1,
            Some('<') => count -= 1,
            _ => return Ok(ASTNode::DP(count)),
        };
        tokenizer.consume();
    }
}

fn parse_loop(tokenizer: &mut Tokenizer) -> Result<ASTNode, &'static str> {
    let mut body = Vec::new();
    tokenizer.consume();
    loop {
        match tokenizer.token {
            Some(']') => break,
            Some(_) => body.push(parse_token(tokenizer)?),
            None => return Err("Unmatched ["),
        };
    }
    tokenizer.consume();
    Ok(ASTNode::Loop(Box::new(ASTNode::Body(Box::new(body)))))
}

struct Tokenizer<'a> {
    token: Option<char>,
    iterator: Chars<'a>,
}

impl<'a> Tokenizer<'a> {
    fn new(program: &'a str) -> Tokenizer<'a> {
        let mut tk = Tokenizer{ iterator: program.chars(),  token: None };
        tk.consume();
        tk
    }

    fn consume(&mut self){
        loop {
            match self.iterator.next() {
                Some(c) if c == '-' 
                    || c == '+'
                    || c == '['
                    || c == ']'
                    || c == '<'
                    || c == '>'
                    || c == ','
                    || c == '.' => {
                        self.token = Some(c);
                        return;
                    },
                Some(_) => {},
                None => {
                    self.token = None;
                    return;
                    }
            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenizer_test() {
        let program = "++-[ butts\n>\n <,.]asdfe";
        let mut tokenizer = Tokenizer::new(program);
        
        assert_eq!(tokenizer.token, Some('+'));
        tokenizer.consume();
        assert_eq!(tokenizer.token, Some('+'));
        tokenizer.consume();
        assert_eq!(tokenizer.token, Some('-'));
        tokenizer.consume();
        assert_eq!(tokenizer.token, Some('['));
        tokenizer.consume();
        assert_eq!(tokenizer.token, Some('>'));
        tokenizer.consume();
        assert_eq!(tokenizer.token, Some('<'));
        tokenizer.consume();
        assert_eq!(tokenizer.token, Some(','));
        tokenizer.consume();
        assert_eq!(tokenizer.token, Some('.'));
        tokenizer.consume();
        assert_eq!(tokenizer.token, Some(']'));
        tokenizer.consume();
        assert_eq!(tokenizer.token, None);
    }

    #[test]
    #[should_panic]
    fn no_empty_programs() {
        let program = "";
        parse(program).unwrap();
    }

    #[test]
    fn parse_no_loops() {
        let program = "+-><,.";
        let ast = ASTNode::Body(Box::new(vec![
                ASTNode::ADD(0),
                ASTNode::DP(0),
                ASTNode::GETC,
                ASTNode::PUTC,
            ]));
        assert_eq!(ast, parse(program).unwrap());
    }

    #[test]
    fn parse_loops() {
        let program = "+[>---]+";
        let ast = ASTNode::Body(Box::new(vec![
                ASTNode::ADD(1),
                ASTNode::Loop(Box::new(ASTNode::Body(Box::new(vec![
                    ASTNode::DP(1),
                    ASTNode::ADD(-3),
                ])))),
                ASTNode::ADD(1),
            ]));
        assert_eq!(ast, parse(program).unwrap());
    }
}
