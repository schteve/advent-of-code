/*
    --- Day 18: Operation Order ---
    As you look out the window and notice a heavily-forested continent slowly appear over the horizon, you are interrupted by the child sitting next to you. They're curious if you could help them with their math homework.

    Unfortunately, it seems like this "math" follows different rules than you remember.

    The homework (your puzzle input) consists of a series of expressions that consist of addition (+), multiplication (*), and parentheses ((...)). Just like normal math, parentheses indicate that the expression inside must be evaluated before it can be used by the surrounding expression. Addition still finds the sum of the numbers on both sides of the operator, and multiplication still finds the product.

    However, the rules of operator precedence have changed. Rather than evaluating multiplication before addition, the operators have the same precedence, and are evaluated left-to-right regardless of the order in which they appear.

    For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are as follows:

    1 + 2 * 3 + 4 * 5 + 6
    3   * 3 + 4 * 5 + 6
        9   + 4 * 5 + 6
            13   * 5 + 6
                65   + 6
                    71
    Parentheses can override this order; for example, here is what happens if parentheses are added to form 1 + (2 * 3) + (4 * (5 + 6)):

    1 + (2 * 3) + (4 * (5 + 6))
    1 +    6    + (4 * (5 + 6))
        7      + (4 * (5 + 6))
        7      + (4 *   11   )
        7      +     44
                51
    Here are a few more examples:

    2 * 3 + (4 * 5) becomes 26.
    5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
    5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
    ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
    Before you can help with the homework, you need to understand it yourself. Evaluate the expression on each line of the homework; what is the sum of the resulting values?

    --- Part Two ---
    You manage to answer the child's questions and they finish part 1 of their homework, but get stuck when they reach the next section: advanced math.

    Now, addition and multiplication have different precedence levels, but they're not the ones you're familiar with. Instead, addition is evaluated before multiplication.

    For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are now as follows:

    1 + 2 * 3 + 4 * 5 + 6
    3   * 3 + 4 * 5 + 6
    3   *   7   * 5 + 6
    3   *   7   *  11
        21       *  11
            231
    Here are the other examples from above:

    1 + (2 * 3) + (4 * (5 + 6)) still becomes 51.
    2 * 3 + (4 * 5) becomes 46.
    5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 1445.
    5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 669060.
    ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 23340.
    What do you get if you add up the results of evaluating the homework problems using these new rules?
*/

use crate::common::{trim_start, unsigned};
use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, value},
    multi::many1,
    IResult,
};

#[derive(Clone, Copy, PartialEq)]
pub enum Token {
    Number(u64),
    Add,
    Mult,
    ParenOpen,
    ParenClose,
}

impl Token {
    fn parser(input: &str) -> IResult<&str, Self> {
        trim_start(alt((
            map(unsigned, Self::Number),
            value(Self::Add, char('+')),
            value(Self::Mult, char('*')),
            value(Self::ParenOpen, char('(')),
            value(Self::ParenClose, char(')')),
        )))(input)
    }

    fn is_op(&self) -> bool {
        matches!(self, Self::Add | Self::Mult)
    }
}

pub struct ExprInfix {
    tokens: Vec<Token>,
}

impl ExprInfix {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, tokens) = many1(Token::parser)(input)?;
        Ok((input, Self { tokens }))
    }

    fn to_postfix(&self, precedence: fn(&Token) -> u32) -> ExprPostfix {
        let mut postfix: Vec<Token> = Vec::new();
        let mut op_stack: Vec<Token> = Vec::new();
        for &t in &self.tokens {
            match t {
                Token::Number(_) => postfix.push(t),
                Token::Add | Token::Mult => {
                    while let Some(op) = op_stack.pop() {
                        if op.is_op() == true && precedence(&t) <= precedence(&op) {
                            postfix.push(op);
                        } else {
                            op_stack.push(op);
                            break;
                        }
                    }
                    op_stack.push(t);
                }
                Token::ParenOpen => op_stack.push(t),
                Token::ParenClose => {
                    while let Some(op) = op_stack.pop() {
                        if op == Token::ParenOpen {
                            break;
                        } else {
                            postfix.push(op);
                        }
                    }
                }
            }
        }
        while let Some(t) = op_stack.pop() {
            postfix.push(t);
        }

        ExprPostfix { tokens: postfix }
    }
}

struct ExprPostfix {
    tokens: Vec<Token>,
}

impl ExprPostfix {
    fn evaluate(&self) -> u64 {
        let mut stack: Vec<u64> = Vec::new();
        for t in &self.tokens {
            match t {
                Token::Number(x) => stack.push(*x),
                Token::Add => {
                    let value = stack.pop().unwrap() + stack.pop().unwrap();
                    stack.push(value);
                }
                Token::Mult => {
                    let value = stack.pop().unwrap() * stack.pop().unwrap();
                    stack.push(value);
                }
                Token::ParenOpen | Token::ParenClose => panic!("Unexpected parenthesis"),
            }
        }
        assert_eq!(stack.len(), 1);
        stack.pop().unwrap()
    }
}

fn precedence1(t: &Token) -> u32 {
    match t {
        Token::Add => 0,
        Token::Mult => 0,
        _ => panic!("No precedence defined"),
    }
}

fn precedence2(t: &Token) -> u32 {
    match t {
        Token::Add => 1,
        Token::Mult => 0,
        _ => panic!("No precedence defined"),
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<ExprInfix> {
    input
        .lines()
        .map(|line| ExprInfix::parser(line).unwrap().1)
        .collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &[ExprInfix]) -> u64 {
    let sum: u64 = input
        .iter()
        .map(|expr| expr.to_postfix(precedence1).evaluate())
        .sum();
    assert_eq!(sum, 280014646144);
    sum
}

#[aoc(day18, part2)]
pub fn part2(input: &[ExprInfix]) -> u64 {
    let sum: u64 = input
        .iter()
        .map(|expr| expr.to_postfix(precedence2).evaluate())
        .sum();
    assert_eq!(sum, 9966990988262);
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn test_evaluate() {
        let expressions = input_generator(EXAMPLE_INPUT);
        let results: Vec<u64> = expressions
            .iter()
            .map(|expr| expr.to_postfix(precedence1).evaluate())
            .collect();
        let expected = [71, 51, 26, 437, 12240, 13632];
        assert_eq!(results, expected);

        let expressions = input_generator(EXAMPLE_INPUT);
        let results: Vec<u64> = expressions
            .iter()
            .map(|expr| expr.to_postfix(precedence2).evaluate())
            .collect();
        let expected = [231, 51, 46, 1445, 669060, 23340];
        assert_eq!(results, expected);
    }
}
