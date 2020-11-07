use super::lang::*;
use anyhow::Result;

pub(crate) fn parse_input(s: &str) -> Result<Vec<Segment<'_>>> {
    s.lines().map(parse_line).collect::<Result<Vec<_>>>()
}

fn parse_line(s: &str) -> Result<Segment<'_>> {
    let tokens = tokenize(s).collect::<Vec<_>>();
    let mut rtokens = tokens.iter().rev();
    let output = parse_identifier(rtokens.next().unwrap());
    parse_assignment(rtokens.next().unwrap());
    let rhs = parse_input_token(rtokens.next().unwrap());

    let seg = match rtokens.next() {
        None => Segment {circut: Circut::Simple(rhs), output},
        Some(Token::Op(Operation::Not)) => Segment {circut: Circut::Not(rhs), output},
        Some(Token::Op(op)) => {
            let lhs = parse_input_token(rtokens.next().unwrap());
            match op {
                Operation::And => Segment {circut: Circut::And(lhs, rhs), output},
                Operation::Or => Segment {circut: Circut::Or(lhs, rhs), output},
                Operation::LeftShift => Segment {circut: Circut::LeftShift(lhs, rhs), output},
                Operation::RightShift => Segment {circut: Circut::RightShift(lhs, rhs), output},
                _ => panic!("unexpected op: {:?} {:?}", op, s),
            }
        }
        Some(t) => panic!("unexpected token: {:?} in {:?}", t, s),
    };

    Ok(seg)
}

fn parse_input_token<'a>(t: &Token<'a>) -> Input<'a> {
    match t {
        Token::Identifier(id) => Input::Wire(*id),
        Token::Literal(l) => Input::Literal(*l),
        _ => panic!("expected input from: {:?}", t),
    }
}

fn parse_identifier<'a>(t: &Token<'a>) -> Identifier<'a> {
    match t {
        Token::Identifier(id) => *id,
        _ => panic!("expected identifier: {:?}", t),
    }
}

fn parse_assignment<'a>(t: &Token<'a>) {
    match t {
        Token::Assign => {}, 
        _ => panic!("expected assignment: {:?}", t),
    }
}

fn tokenize<'a>(s: &'a str) -> impl Iterator<Item = Token<'a>> + 'a {
    s.split_ascii_whitespace().map(move |t| match t {
        "->" => Token::Assign,
        "AND" => Token::Op(Operation::And),
        "OR" => Token::Op(Operation::Or),
        "NOT" => Token::Op(Operation::Not),
        "LSHIFT" => Token::Op(Operation::LeftShift),
        "RSHIFT" => Token::Op(Operation::RightShift),
        _ => {
            if let Some(literal) = t.parse::<u16>().ok() {
                Token::Literal(Literal(literal))
            } else {
                Token::Identifier(Identifier(t))
            }
        }
    })
}
