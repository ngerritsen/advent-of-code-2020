use std::vec;

enum Op {
    Add,
    Mul,
    Num(u64),
}

struct Expr {
    children: Vec<Expr>,
    op: Op,
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    let tot: u64 = input.iter().map(|c| eval(c, 0).0).sum();
    let tot_advanced: u64 = input.iter().map(|c| eval_advanced(&parse(c))).sum();

    println!("{}", tot);
    println!("{}", tot_advanced);
}

fn eval_advanced(expr: &Expr) -> u64 {
    match expr.op {
        Op::Num(x) => x,
        Op::Add => expr.children.iter().map(eval_advanced).sum(),
        Op::Mul => expr.children.iter().map(eval_advanced).product(),
    }
}

fn eval(calc: &Vec<char>, mut i: usize) -> (u64, usize) {
    let mut ans = 0;
    let mut op = '+';

    while i < calc.len() {
        let tok = calc[i];

        if tok.is_numeric() {
            ans = apply(op, tok.to_digit(10).unwrap() as u64, ans);
        } else if tok == '(' {
            let (num, next_i) = eval(calc, i + 1);
            ans = apply(op, num, ans);
            i = next_i
        } else if tok == ')' {
            return (ans, i);
        } else {
            op = tok;
        }

        i += 1;
    }

    (ans, i)
}

fn parse(calc: &Vec<char>) -> Expr {
    let (expr, _) = parse_mul_expr(calc, 0);
    expr
}

fn parse_mul_expr(calc: &Vec<char>, i: usize) -> (Expr, usize) {
    let (first, mut i) = parse_add_expr(calc, i);
    let mut expr = Expr::from_op(Op::Mul, first);

    while i < calc.len() && calc[i] == '*' {
        let (next_expr, next_i) = parse_add_expr(calc, i + 1);
        i = next_i;
        expr.children.push(next_expr);
    }

    (expr, i)
}

fn parse_add_expr(calc: &Vec<char>, i: usize) -> (Expr, usize) {
    let (first, mut i) = parse_num_expr(calc, i);
    let mut expr = Expr::from_op(Op::Add, first);

    while i < calc.len() && calc[i] == '+' {
        let (next_expr, next_i) = parse_num_expr(calc, i + 1);
        i = next_i;
        expr.children.push(next_expr);
    }

    (expr, i)
}

fn parse_num_expr(calc: &Vec<char>, i: usize) -> (Expr, usize) {
    if calc[i] == '(' {
        let (expr, i) = parse_mul_expr(calc, i + 1);
        return (expr, i + 1);
    }

    let num = calc[i].to_digit(10).unwrap() as u64;
    (Expr::from_val(num), i + 1)
}

fn apply(op: char, a: u64, b: u64) -> u64 {
    match op {
        '+' => a + b,
        '*' => a * b,
        _ => panic!("Invalid operator"),
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().filter(|c| *c != ' ').collect())
        .collect()
}

impl Expr {
    fn from_op(op: Op, first: Expr) -> Expr {
        Expr {
            children: vec![first],
            op,
        }
    }

    fn from_val(val: u64) -> Expr {
        Expr {
            children: Vec::new(),
            op: Op::Num(val),
        }
    }
}
