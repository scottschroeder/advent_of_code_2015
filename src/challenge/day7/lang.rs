use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Literal(pub u16);

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Identifier<'a>(pub &'a str);

impl<'a> fmt::Debug for Identifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    And,
    Or,
    Not,
    LeftShift,
    RightShift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    Identifier(Identifier<'a>),
    Literal(Literal),
    Op(Operation),
    Assign,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input<'a> {
    Literal(Literal),
    Wire(Identifier<'a>),
}


impl<'a> Input<'a> {
    fn as_wire(&self) -> Option<Identifier<'a>> {
        match self {
            Input::Wire(w) => Some(*w),
            Input::Literal(_) => None,
        }
    }
    fn resolve(&self, wires: &HashMap<Identifier<'a>, u16>) -> Option<u16> {
        match self {
            Input::Literal(l) => Some(l.0),
            Input::Wire(w) => wires.get(w).cloned(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Circut<'a> {
    Simple(Input<'a>),
    Not(Input<'a>),
    And(Input<'a>, Input<'a>),
    Or(Input<'a>, Input<'a>),
    LeftShift(Input<'a>, Input<'a>),
    RightShift(Input<'a>, Input<'a>),
}

impl<'a> Circut<'a> {
    pub fn resolve(&self, wires: &HashMap<Identifier<'a>, u16>) -> Option<u16> {
        match self {
            Circut::Simple(lhs) => lhs.resolve(wires),
            Circut::Not(lhs) => lhs.resolve(wires).map(|s| !s),
            Circut::And(lhs, rhs) => lhs
                .resolve(wires)
                .zip(rhs.resolve(wires))
                .map(|(l, r)| l & r),
            Circut::Or(lhs, rhs) => lhs
                .resolve(wires)
                .zip(rhs.resolve(wires))
                .map(|(l, r)| l | r),
            Circut::LeftShift(lhs, rhs) => lhs
                .resolve(wires)
                .zip(rhs.resolve(wires))
                .map(|(l, r)| l << r),
            Circut::RightShift(lhs, rhs) => lhs
                .resolve(wires)
                .zip(rhs.resolve(wires))
                .map(|(l, r)| l >> r),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Segment<'a> {
    pub circut: Circut<'a>,
    pub output: Identifier<'a>,
}

impl<'a> Segment<'a> {
    pub fn inputs(&self) -> Inputs<'a> {
        match self.circut {
            Circut::Simple(lhs) => Inputs::one(lhs),
            Circut::Not(lhs) => Inputs::one(lhs),
            Circut::And(lhs, rhs) => Inputs::two(lhs, rhs),
            Circut::Or(lhs, rhs) => Inputs::two(lhs, rhs),
            Circut::LeftShift(lhs, rhs) => Inputs::two(lhs, rhs),
            Circut::RightShift(lhs, rhs) => Inputs::two(lhs, rhs),
        }
    }
}

pub struct Inputs<'a> {
    first: Option<Identifier<'a>>,
    second: Option<Identifier<'a>>,
}
impl<'a> Inputs<'a> {
    fn one(id: Input<'a>) -> Inputs<'a> {
        Inputs {
            first: id.as_wire(),
            second: None,
        }
    }
    fn two(lhs: Input<'a>, rhs: Input<'a>) -> Inputs<'a> {
        let first = lhs.as_wire();
        if let Some(first) = first {
            Inputs {
                first: Some(first),
                second: rhs.as_wire(),
            }
        } else {
            Inputs {
                first: rhs.as_wire(),
                second: None,
            }
        }
    }
}

impl<'a> Iterator for Inputs<'a> {
    type Item = Identifier<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let output = self.first.take();
        self.first = self.second.take();
        output
    }
}
