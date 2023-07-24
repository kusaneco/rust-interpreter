use crate::parser::Parser;
use crate::Expr;
use crate::Statement;
use crate::Token;

impl Parser {
    ///
    /// Statement = IfStatement | AssignStatement | CompoundStatement
    /// CompoundStatement = Statement { ';' Statement }
    ///
    pub(crate) fn state(&mut self) -> Statement {
        let result = match self.current() {
            Some(Token::FUNC) => self.func_statement(), 
            Some(Token::RETURN) => self.return_statement(), 
            Some(Token::IF) => self.if_statement(),
            Some(Token::IDENT(s)) => 
                match self.next() {
                    Some(Token::EQ) => self.ident_statement(s), 
                    _ => self.expr_statement()
                }
            _ => self.expr_statement(),
        };

        // 文の後に ';' が続くようであれば次の文を扱う
        match self.current() {
            Some(Token::SEMICOLON) => self.compound_statement(result),
            _ => result,
        }
    }

    pub(crate) fn expr(&mut self) -> Expr {
        return self.add();
    }

    fn print(&mut self) -> Statement {
        self.confirm(Token::FUNC);
        Statement::Print { expr: self.expr() }
    }

    fn func_statement(&mut self) -> Statement {
        self.confirm(Token::FUNC);
        // name
        let name = match self.current() {
            Some(Token::IDENT(s)) => s,
            _ => panic!("unexpected identifier"),
        };
        self.fix();

        self.confirm(Token::LPAR);
        // args
        let mut args = vec![];
        loop {
            match self.current() {
                Some(Token::IDENT(s)) => {
                    args.push(s);
                    self.fix();
                }
                _ => {
                    break;
                }
            }
        }
        self.confirm(Token::RPAR);

        self.confirm(Token::LBRACE);
        // body
        let body = self.state();
        self.confirm(Token::RBRACE);
        Statement::Func {
            name: name,
            args: args,
            body: Box::new(body),
        }
    }

    fn return_statement(&mut self) -> Statement {
        self.confirm(Token::RETURN);
        Statement::Return { expr: self.expr() }
    }

    fn if_statement(&mut self) -> Statement {
        self.confirm(Token::IF);
        let expr1 = self.expr();

        self.confirm(Token::LBRACE);
        let state1 = self.state();
        self.confirm(Token::RBRACE);

        self.confirm(Token::ELSE);
        self.confirm(Token::LBRACE);
        let state2 = self.state();
        self.confirm(Token::RBRACE);

        Statement::If {
            condition: Box::new(expr1),
            then: Box::new(state1),
            els: Box::new(state2),
        }
    }

    fn ident_statement(&mut self, s: String) -> Statement {
        match self.next() {
            Some(Token::EQ) => {
                self.fix();
                self.confirm(Token::EQ);
                Statement::Assign {
                    id: s,
                    e: Box::new(self.expr()),
                }
            }
            _ => Statement::Null,
        }
    }

    fn expr_statement(&mut self) -> Statement {
        Statement::ExprStatement { expr: self.expr() }
    }

    fn compound_statement(&mut self, st: Statement) -> Statement {
        self.confirm(Token::SEMICOLON);
        Statement::CompoundStatement {
            st1: Box::new(st),
            st2: Box::new(self.state()),
        }
    }
}
