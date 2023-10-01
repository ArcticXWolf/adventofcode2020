// Trying my hand at a compiler:
// tokenizer -> module tokenizer with nom
// parser -> shunting yard algorithm in ASTNode
// evaluation (instead of code generation) -> evaluate() in ASTNode

mod tokenizer {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, multispace0},
        combinator::into,
        error::Error as NomError,
        multi::many1,
        sequence::delimited,
        Finish, IResult,
    };

    use crate::Token;

    pub fn parse_tokens(s: &str) -> Result<Vec<Token>, NomError<&str>> {
        let (_, x) = many1(parse_token)(s).finish()?;
        Ok(x)
    }

    fn parse_token(s: &str) -> IResult<&str, Token> {
        into(delimited(multispace0, parse_token_str, multispace0))(s)
    }

    fn parse_token_str(s: &str) -> IResult<&str, &str> {
        alt((digit1, tag("("), tag(")"), tag("+"), tag("*")))(s)
    }
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Number(usize),
    AdditionOperator,
    MultiplicationOperator,
    OpenParenthesis,
    CloseParenthesis,
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            "(" => Self::OpenParenthesis,
            ")" => Self::CloseParenthesis,
            "+" => Self::AdditionOperator,
            "*" => Self::MultiplicationOperator,
            x => Self::Number(x.parse().unwrap()),
        }
    }
}

impl Token {
    pub fn precedence(&self, is_part1: bool) -> usize {
        if is_part1 {
            return match self {
                Token::AdditionOperator => 100,
                Token::MultiplicationOperator => 100,
                Token::CloseParenthesis => 999999,
                Token::OpenParenthesis => 999999,
                _ => unreachable!(),
            };
        }
        match self {
            Token::AdditionOperator => 200,
            Token::MultiplicationOperator => 100,
            Token::CloseParenthesis => 999999,
            Token::OpenParenthesis => 999999,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum ASTNode {
    Leaf(usize),
    Addition(Box<ASTNode>, Box<ASTNode>),
    Multiplication(Box<ASTNode>, Box<ASTNode>),
}

// for all parts
impl ASTNode {
    pub fn evaluate(&self) -> usize {
        match self {
            Self::Leaf(n) => *n,
            Self::Addition(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Self::Multiplication(lhs, rhs) => lhs.evaluate() * rhs.evaluate(),
        }
    }

    pub fn shunting_yard_algorithm(mut tokens: Vec<Token>, is_part1: bool) -> Self {
        tokens.push(Token::CloseParenthesis);
        tokens.reverse();
        tokens.push(Token::OpenParenthesis);

        let mut operator_stack: Vec<Token> = vec![];
        let mut expression_stack: Vec<Self> = vec![];
        while let Some(current_token) = tokens.pop() {
            match current_token {
                Token::OpenParenthesis => operator_stack.push(current_token),
                Token::Number(n) => expression_stack.push(Self::Leaf(n)),
                op @ Token::AdditionOperator | op @ Token::MultiplicationOperator => {
                    while !operator_stack.is_empty() {
                        match operator_stack.last() {
                            Some(Token::OpenParenthesis) => {
                                break;
                            }
                            Some(tmp_op @ Token::AdditionOperator)
                            | Some(tmp_op @ Token::MultiplicationOperator) => {
                                if tmp_op.precedence(is_part1) < op.precedence(is_part1) {
                                    break;
                                }
                            }
                            _ => unreachable!(),
                        }

                        let rhs = expression_stack.pop().unwrap();
                        let lhs = expression_stack.pop().unwrap();
                        expression_stack.push(match operator_stack.pop() {
                            Some(Token::AdditionOperator) => {
                                Self::Addition(Box::new(lhs), Box::new(rhs))
                            }
                            Some(Token::MultiplicationOperator) => {
                                Self::Multiplication(Box::new(lhs), Box::new(rhs))
                            }
                            _ => unreachable!(),
                        });
                    }

                    operator_stack.push(op);
                }
                Token::CloseParenthesis => {
                    while !operator_stack.is_empty() {
                        if let Some(Token::OpenParenthesis) = operator_stack.last() {
                            break;
                        }

                        let rhs = expression_stack.pop().unwrap();
                        let lhs = expression_stack.pop().unwrap();
                        expression_stack.push(match operator_stack.pop() {
                            Some(Token::AdditionOperator) => {
                                Self::Addition(Box::new(lhs), Box::new(rhs))
                            }
                            Some(Token::MultiplicationOperator) => {
                                Self::Multiplication(Box::new(lhs), Box::new(rhs))
                            }
                            _ => unreachable!(),
                        });
                    }

                    operator_stack.pop(); // pop the beginning (
                }
            }
        }

        expression_stack.pop().unwrap()
    }
}

pub fn part_one(_input: &str) -> Option<usize> {
    let mut sum = 0;
    for l in _input.lines() {
        let tokens = tokenizer::parse_tokens(l).unwrap();
        let ast = ASTNode::shunting_yard_algorithm(tokens, true);
        sum += ast.evaluate();
    }

    Some(sum)
}

pub fn part_two(_input: &str) -> Option<usize> {
    let mut sum = 0;
    for l in _input.lines() {
        let tokens = tokenizer::parse_tokens(l).unwrap();
        let ast = ASTNode::shunting_yard_algorithm(tokens, false);
        sum += ast.evaluate();
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(26335));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(693891));
    }
}
