use crate::phenotype::parse::Value;
use crate::error::Error;
use crate::phenotype::parse::tokenize::Token;
use std::fmt::{Display, Formatter};

pub(super) enum Tree {
    Call(Call),
    Value(Value),
}

pub(super) struct Call {
    pub(super) name: String,
    pub(super) args: Vec<Tree>,
}

impl Call {
    fn new(name: String) -> Call {
        let args: Vec<Tree> = Vec::new();
        Call { name, args }
    }
    fn add_arg(&mut self, arg: Tree) {
        self.args.push(arg)
    }
}

enum TreeizeState {
    Start,
    OnValue(Value),
    OnComma,
    OnOpen,
    OnClose,
}

impl TreeizeState {
    fn new() -> TreeizeState { TreeizeState::Start }
    fn next_token(self, stack: &mut Vec<Call>, call: &mut Call, token: Token)
                  -> Result<TreeizeState, Error> {
        match self {
            TreeizeState::Start => {
                match token {
                    Token::OpenParens => { Err(Error::from("Cannot start with '('")) }
                    Token::CloseParen => { Err(Error::from("Cannot start with ')'")) }
                    Token::Comma => { Err(Error::from("Cannot start with ','")) }
                    Token::Value(value) => { Ok(TreeizeState::OnValue(value)) }
                }
            }
            TreeizeState::OnValue(value) => {
                match token {
                    Token::OpenParens => {
                        match value {
                            Value::String(string) => {
                                stack.push(std::mem::replace(call,
                                                             Call::new(string)));
                                Ok(TreeizeState::OnOpen)
                            }
                            Value::Number(number) => {
                                Err(Error::from(format!("'{}' cannot be followed by '('.",
                                                        number)))
                            }
                        }
                    }
                    Token::CloseParen => {
                        call.add_arg(Tree::Value(value));
                        TreeizeState::close_parens(stack, call)
                    }
                    Token::Comma => {
                        call.add_arg(Tree::Value(value));
                        Ok(TreeizeState::OnComma)
                    }
                    Token::Value(value2) => {
                        Err(Error::from(format!("Need to separate '{}' and '{}'.", value,
                                                value2)))
                    }
                }
            }
            TreeizeState::OnComma => {
                match token {
                    Token::OpenParens => { Err(Error::from("Cannot write ',('.")) }
                    Token::CloseParen => { Err(Error::from("Cannot write ',)'.")) }
                    Token::Comma => { Err(Error::from("Cannot write ',,'.")) }
                    Token::Value(value) => { Ok(TreeizeState::OnValue(value)) }
                }
            }
            TreeizeState::OnOpen => {
                match token {
                    Token::OpenParens => { Err(Error::from("Cannot write ',('.")) }
                    Token::CloseParen => { TreeizeState::close_parens(stack, call) }
                    Token::Comma => { Err(Error::from("Cannot write '(,'.")) }
                    Token::Value(value) => { Ok(TreeizeState::OnValue(value)) }
                }
            }
            TreeizeState::OnClose => {
                match token {
                    Token::OpenParens => { Err(Error::from("Cannot write ')('.")) }
                    Token::CloseParen => { TreeizeState::close_parens(stack, call) }
                    Token::Comma => { Ok(TreeizeState::OnComma) }
                    Token::Value(value) => {
                        Err(Error::from(format!("Need comma between ')' and {}", value)))
                    }
                }
            }
        }
    }

    fn close_parens(stack: &mut Vec<Call>, call: &mut Call) -> Result<TreeizeState, Error> {
        match stack.pop() {
            None => { Err(Error::from("')' with no matching '('.")) }
            Some(outer_call) => {
                let inner_call = std::mem::replace(call, outer_call);
                call.add_arg(Tree::Call(inner_call));
                Ok(TreeizeState::OnClose)
            }
        }
    }
    fn end(self, stack: Vec<Call>, mut call: Call) -> Result<Call, Error> {
        match self {
            TreeizeState::Start => {Err(Error::from("Missing phenotype definition"))}
            TreeizeState::OnValue(value) => {
                if !stack.is_empty() {
                    Err(Error::from("'(' with no matching ')'."))
                } else {
                    call.add_arg(Tree::Value(value));
                    Ok(call)
                }
            }
            TreeizeState::OnComma => { Err(Error::from("Cannot end with a ','"))}
            TreeizeState::OnOpen => {Err(Error::from("Cannot end with a '('"))}
            TreeizeState::OnClose => {
                if !stack.is_empty() {
                    Err(Error::from("'(' with no matching ')'."))
                } else {
                    Ok(call)
                }
            }
        }
    }
}

pub(super) fn treeize(tokens: Vec<Token>) -> Result<Call, Error> {
    let mut call = Call::new(String::from("main"));
    let mut call_stack: Vec<Call> = Vec::new();
    let mut state = TreeizeState::new();
    for token in tokens {
        state = state.next_token(&mut call_stack, &mut call, token)?;
    }
    state.end(call_stack, call)
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tree::Call(call) => {
                write!(f, "{}", call)
            }
            Tree::Value(value) => {
                match value {
                    Value::String(string) => { write!(f, "{}", string)}
                    Value::Number(number) => { write!(f, "{}", number)}
                }
            }
        }
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let args = self.args.iter().map(|arg|{
            format!("{}", arg)
        }).collect::<Vec<String>>().join(",");
        write!(f, "{}({})", self.name, args)
    }
}