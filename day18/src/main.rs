use advent::*;

#[derive(Debug, Copy, Clone)]
enum Operator {
    Mul,
    Add
}
#[derive(Debug, Clone)]
enum Expression {
    Block(Vec<Expression>),
    Number(i64),
    Op(Operator),
}

#[derive(Debug, Copy, Clone)]
enum Eval {
    Value(i64),
    Op(Operator)
}

impl Expression {
    fn parse_block<'a>(it: &mut impl Iterator<Item=&'a str>) -> Result<Expression, Error> {
        let mut block = Vec::new();
        while let Some(token) = it.next() {
            let p = match token {
                "*" => Ok(Expression::Op(Operator::Mul)),
                "+" => Ok(Expression::Op(Operator::Add)),
                "(" => Expression::parse_block(it),
                ")" => break,
                x => x.parse::<i64>().map(|x| Expression::Number(x)).map_err(|err| anyhow!( "Could not parse number: {:?}", err))
            };
            block.push(p?);
        }
        Ok(Expression::Block(block))
    }

    fn new(input: &str) -> Result<Expression, Error> {
        let post = input.replace("(", " ( ").replace(")", " ) ");
        Expression::parse_block(&mut post.split_whitespace())
    }

    fn exec(&self) -> Eval {
        match self {
            Expression::Block(ref expr) => {
                let mut acc = 0;
                let mut op = Operator::Add;

                for e in expr.iter() {
                    match e.exec() {
                        Eval::Value(v) => {
                            match op {
                                Operator::Add => acc += v,
                                Operator::Mul => acc *= v
                            }
                        },
                        Eval::Op(o) => op = o,
                    }
                }
                Eval::Value(acc)
            },
            Expression::Number(x) => Eval::Value(*x),
            Expression::Op(op) => Eval::Op(*op),
        }
    }

    fn exec2(&self) -> Eval {
        match self {
            Expression::Block(ref expr) => {
                let mut product = 1;
                for sub_expr in expr.split(|x| match x {
                    Expression::Op(Operator::Mul) => true,
                    _ => false
                }) {
                    let mut acc = 0;
                    let mut op = Operator::Add;
    
                    for e in sub_expr.iter() {
                        match e.exec2() {
                            Eval::Value(v) => {
                                match op {
                                    Operator::Add => acc += v,
                                    Operator::Mul => acc *= v
                                }
                            },
                            Eval::Op(o) => op = o,
                        }
                    }
                    product *= acc;
                }
                Eval::Value(product)
            },
            x => x.exec(),
        }
    }


    fn eval(&self) -> i64 {
        match self.exec() {
            Eval::Value(x) => x,
            Eval::Op(_) => 0
        }
    }
    fn eval2(&self) -> i64 {
        match self.exec2() {
            Eval::Value(x) => x,
            Eval::Op(_) => 0
        }
    }
}

fn main() -> Result<(), Error> {
    let input = std::fs::read_to_string("day18/input.txt")?;

    let mut sum = 0;
    for line in input.lines() {
        let expr = Expression::new(line)?;
        sum += expr.eval();
    }
    println!( "{}", sum );

    let mut sum = 0;
    for line in input.lines() {
        let expr = Expression::new(line)?;
        sum += expr.eval2();
    }
    println!( "{}", sum );

    Ok(())
}
