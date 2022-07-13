extern crate core;

use std::fmt::Formatter;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Clone, Debug)]
enum NodeKind {
    ROOT,
    S,
    K,
    I,
    IDENT(String),
}

impl std::fmt::Display for NodeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use NodeKind::*;
        match self {
            ROOT => {
                write!(f, "#")
            }
            IDENT(s) => {
                write!(f, "\"{}\"", s)
            }
            _ => {
                std::fmt::Debug::fmt(self, f)
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Node {
    kind: NodeKind,
    children: Vec<Node>,
}

impl Node {
    pub fn apply(&mut self) {
        self.normalize();
        if let Some(it) = self.children.pop() {
            match it.kind {
                NodeKind::S => {
                    let x = self.children.pop().unwrap();
                    let y = self.children.pop().unwrap();
                    let z = self.children.pop().unwrap();
                    let mut yz = y.clone();
                    yz.children.reverse();
                    yz.children.push(z.clone());
                    yz.children.reverse();
                    self.children.push(yz);
                    self.children.push(z);
                    self.children.push(x);
                }
                NodeKind::K => {
                    let x = self.children.pop().unwrap();
                    let _ = self.children.pop().unwrap();
                    self.children.push(x);
                }
                NodeKind::I => {}
                NodeKind::ROOT => {}
                NodeKind::IDENT(_) => {
                    self.apply();
                    self.children.push(it);
                }
            }
        };
        self.normalize()
    }
    pub fn normalize(&mut self) {
        if let Some(it) = self.children.pop() {
            let new_last = Node { kind: it.kind, children: vec![] };
            it.children.into_iter().for_each(|n| self.children.push(n));
            self.children.push(new_last);
        }
    }

    pub fn s(children: Vec<Node>) -> Self {
        Node {
            kind: NodeKind::S,
            children,
        }
    }
    pub fn k(children: Vec<Node>) -> Self {
        Node {
            kind: NodeKind::K,
            children,
        }
    }
    pub fn i(children: Vec<Node>) -> Self {
        Node {
            kind: NodeKind::I,
            children,
        }
    }
    pub fn new(ident: String, children: Vec<Node>) -> Self {
        Node {
            kind: NodeKind::IDENT(ident),
            children,
        }
    }
    fn parse_slice_chars(chars: &[char], i: &mut usize) -> Result<Self, ()> {
        let mut nodes = vec![];
        while *i < chars.len() {
            nodes.push(
                match chars[*i] {
                    'S' => Node::s(vec![]),
                    'K' => Node::k(vec![]),
                    'I' => Node::i(vec![]),
                    '(' => {
                        *i += 1;
                        Self::parse_slice_chars(&chars, i)?
                    }
                    ')' => {
                        break;
                    }
                    c => Node::new(String::from(c), vec![]),
                }
            );
            *i += 1;
        }
        nodes.reverse();
        if let Some(mut first) = nodes.pop() {
            first.children = nodes;
            Ok(first)
        } else {
            Err(())
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.children.is_empty() {
            write!(f, "{}", self.kind)
        } else {
            write!(f, "({}{})", self.kind, self.children.iter().rev().map(|n| format!("{}", n)).join(""))
        }
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut children = Self::parse_slice_chars(&s.chars().collect_vec(), &mut 0)?;
        let mut node = Node {
            kind: NodeKind::ROOT,
            children: vec![children],
        };
        node.normalize();
        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::Node;
    use crate::NodeKind::*;

    #[test]
    fn it_works() {
        let mut root = Node {
            kind: ROOT,
            children: vec![
                Node::s(vec![]),
                Node::i(vec![]),
                Node::i(vec![]),
                Node::new(String::from("a"), vec![]),
            ].into_iter().rev().collect(),
        };
        for _ in 0..5 {
            println!("{}", root);
            root.apply();
        }
        println!("{}", root);
    }

    #[test]
    fn parse() -> Result<(), ()> {
        let mut root = Node::from_str("S(K(SII))(S(S(KS)K)(K(SII)))I")?;
        for _ in 0..1000 {
            println!("{}", root);
            root.apply()
        }
        println!("{}", root);
        Ok(())
    }
}
