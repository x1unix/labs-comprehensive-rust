/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// A literal value
    Value(i64),
}

/// The result of evaluating an expression.
#[derive(Debug, PartialEq, Eq)]
enum Res {
    /// Evaluation was successful, with the given result.
    Ok(i64),
    /// Evaluation failed, with the given error message.
    Err(String),
}

// Allow `Ok` and `Err` as shorthands for `Res::Ok` and `Res::Err`.
use Res::{Err, Ok};

fn eval(e: Expression) -> Res {
    match e {
        Expression::Value(v) => Res::Ok(v),
        Expression::Op { op, left, right } => {
            let left_res = eval(*left);
            let right_res = eval(*right);

            match (left_res, right_res) {
                (Ok(a), Ok(b)) => apply_op(op, a, b),
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
        }
    }
}

fn apply_op(op: Operation, left: i64, right: i64) -> Res {
    match op {
        Operation::Add => Ok(left + right),
        Operation::Sub => Ok(left - right),
        Operation::Mul => Ok(left * right),
        Operation::Div => {
            if right == 0 {
                Err(String::from("division by zero"))
            } else {
                Ok(left / right)
            }
        }
    }
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(85)
    );
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("division by zero"))
    );
}
