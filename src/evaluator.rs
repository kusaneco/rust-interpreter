use crate::BinOp;
use crate::Env;
use crate::FunctionTable;
use crate::Declaration;
use crate::Expr;
use crate::Statement;
use crate::Syntax;

pub fn eval(syntax: Syntax, env: &mut Env, ft: &mut FunctionTable) -> () {
    match syntax {
        Syntax::Statement(st) => {
            exec(st, env, ft);
        }
    }
}

// 文を実行する
fn exec(statement: Statement, env: &mut Env, ft: &mut FunctionTable) -> () {
    match statement {
        Statement::ExprStatement { expr } => {
            println!("{:?\n}", calc(expr, env, ft));  // TODO 出力の仕方を現状に合わせてるだけなので自然な形にする
        }
        Statement::CompoundStatement { st1, st2 } => {
            exec(*st1, env, ft);
            exec(*st2, env, ft);
        }
        // TODO 関数作ったら消す
        Statement::Print { expr } => {
            print!("{:?}\n", calc(expr, env, ft));
        }
        Statement::FunctionDefine { id, params, body } => {
            ft.insert(id, Declaration::Function { params, body });
        }
        Statement::FunctionCall { expr } => {
            calc(expr, env, ft);
        }
        Statement::Return { expr } => {
            let value = calc(*expr, env, ft);
            env.insert("return".to_string(), value);
        }
        Statement::Assign { id, e } => {
            let value = calc(*e, env, ft);
            env.insert(id, value);
        }
        Statement::If {
            condition,
            then,
            els,
        } => {
            if calc(*condition, env, ft) > 0 {
                exec(*then, env, ft)
            } else {
                exec(*els, env, ft)
            }
        }
        _ => panic!("実行できない Statement {:?} を実行しようとした", statement),
    }
}

// 式を計算する
fn calc(expr: Expr, env: &mut Env, ft: &mut FunctionTable) -> i32 {
    match expr {
        Expr::Binary { op, lhs, rhs } => match op {
            BinOp::Add => return calc(*lhs, env, ft) + calc(*rhs, env, ft),
            BinOp::Sub => return calc(*lhs, env, ft) - calc(*rhs, env, ft),
            BinOp::Mul => return calc(*lhs, env, ft) * calc(*rhs, env, ft),
            BinOp::Div => return calc(*lhs, env, ft) / calc(*rhs, env, ft),
        },
        Expr::Number(n) => n,
        Expr::Var(s) => match env.get(&s.to_string()) {
            Some(num) => *num,
            None => panic!("環境 env に変数名 {:?} が登録されていない", &s.to_string()),
        },
        Expr::FunctionCall { id, args } => {
            let mut lenv = Env::new();
            match ft.get(&id.to_string()) {
                Some(Declaration::Function { params, body }) => {
                    for (param, arg) in params.iter().zip(args.clone().iter()) {
                        lenv.insert(param.to_string(), calc(arg.clone(), env, &mut ft.clone()));
                    }

                    exec(*body.clone(), &mut lenv, &mut ft.clone());
                    match lenv.get(&"return".to_string()) {
                        Some(i) => *i,
                        None => todo!("関数が値を返さない場合の挙動が未定義"),
                    }
                }
                None => panic!("関数テーブル ft に関数名 {:?} が登録されていない", id),
            }
        },
    }
}
