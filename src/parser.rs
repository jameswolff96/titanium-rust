use syn::{Ident, Lit, LitInt, Token, parse::{Parse, ParseStream}};

syn::custom_keyword!(exit);
syn::custom_keyword!(print);

pub struct ExitStatement {
    pub exit_token: exit,
    pub value: LitInt,
    pub semi_token: Token![;],
}

impl Parse for ExitStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ExitStatement {
            exit_token: input.parse()?,
            value: input.parse()?,
            semi_token: input.parse()?
        })
    }
}

pub struct LetStatement {
    pub let_token: Token![let],
    pub name: Ident,
    pub eq_token: Token![=],
    pub value: Lit,
    pub semi_token: Token![;],
}

impl Parse for LetStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(LetStatement {
            let_token: input.parse()?,
            name: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
            semi_token: input.parse()?
        })
    }
}

pub enum PrintValue {
    Literal(Lit),
    Variable(Ident),
}

impl Parse for PrintValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Lit) {
            Ok(PrintValue::Literal(input.parse()?))
        } else if input.peek(Ident) {
            Ok(PrintValue::Variable(input.parse()?))
        } else {
            Err(input.error("Expected a literal value or a variable identifier"))
        }
    }
}

pub struct PrintStatement {
    pub print_token: print,
    pub value: PrintValue,
    pub semi_token: Token![;],
}

impl Parse for PrintStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(PrintStatement {
            print_token: input.parse()?,
            value: input.parse()?,
            semi_token: input.parse()?
        })
    }
}

pub enum Statement {
    Exit(ExitStatement),
    Let(LetStatement),
    Print(PrintStatement),
}

impl Parse for Statement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(exit) {
            let stmt = input.parse()?;
            Ok(Statement::Exit(stmt))
        } else if input.peek(Token![let]) {
            let stmt = input.parse()?;
            Ok(Statement::Let(stmt))
        } else if input.peek(print) {
            let stmt = input.parse()?;
            Ok(Statement::Print(stmt))
        } else {
            Err(input.error("Expected 'exit' statement or assignment"))
        }
    }
}

pub struct Program(pub Vec<Statement>);

impl Parse for Program {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(parse_zero_or_more(input)))
    }
}

fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut result = Vec::new();
    while let Ok(item) = input.parse() {
        result.push(item);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_parse {
        ($source:expr, $T:ty) => {{
            let result: Result<$T, syn::Error> = syn::parse_str($source);
            match result {
                Ok(ast) => ast,
                Err(err) => panic!("Failed to parse source: \n\"{}\"\nError: {}", $source, err),
            }
        }};
    }

    #[test]
    fn test_parse_exit_statement() {
        let stmt = assert_parse!("exit 0;", ExitStatement);
        assert_eq!(stmt.value.base10_parse::<i32>().unwrap(), 0);
    }

    #[test]
    fn test_parse_let_statement() {
        let stmt = assert_parse!("let test = 0;", LetStatement);
        assert_eq!(stmt.name.to_string().as_str(), "test");
        if let syn::Lit::Int(lit_int) = stmt.value {
            assert_eq!(lit_int.base10_parse::<i32>().unwrap(), 0);
        } else {
            panic!("Expected an integer literal inside LetStatement")
        }
    }

    #[test]
    fn test_parse_let_statement_str() {
        let stmt = assert_parse!("let test = \"hello world\";", LetStatement);
        assert_eq!(stmt.name.to_string().as_str(), "test");
        if let syn::Lit::Str(lit_str) = stmt.value {
            assert_eq!(lit_str.value(), "hello world");
        } else {
            panic!("Expected a string literal inside LetStatement");
        }
    }

    #[test]
    fn test_parse_print_statement_literal() {
        let stmt = assert_parse!("print \"hello world\";", PrintStatement);
        if let PrintValue::Literal(lit) = stmt.value {
            if let syn::Lit::Str(lit_str) = lit {
                assert_eq!(lit_str.value(), "hello world");
            } else {
                panic!("Expected a string literal inside PrintStatement");
            }
        } else {
            panic!("Expected a literal inside PrintStatement");
        }
    }

    #[test]
    fn test_parse_print_statement_variable() {
        let stmt = assert_parse!("print test;", PrintStatement);
        if let PrintValue::Variable(ident) = stmt.value {
            assert_eq!(ident.to_string(), "test");
        } else {
            panic!("Expected a variable identifier inside PrintStatement");
        }
    }

    #[test]
    fn test_parse_full_program() {
        let program = assert_parse!("print 42;\nlet test = \"hello world\";\nprint test;\nexit 1;", Program);
        let statements = program.0;

        assert_eq!(statements.len(), 4);

        match &statements[0] {
            Statement::Print(p) => {
                if let PrintValue::Literal(lit) = &p.value {
                    if let syn::Lit::Int(lit_int) = lit {
                        assert_eq!(lit_int.base10_parse::<i32>().unwrap(), 42);
                    } else {
                        panic!("Expected an integer literal inside PrintStatement");
                    }
                } else {
                    panic!("Expected a literal inside PrintStatement");
                }
            }
            _ => panic!("Expected first statement to be a Print variant"),
        }

        match &statements[1] {
            Statement::Let(l) => {
                assert_eq!(l.name.to_string().as_str(), "test");
                if let syn::Lit::Str(lit_str) = &l.value {
                    assert_eq!(lit_str.value(), "hello world");
                } else {
                    panic!("Expected a string literal inside LetStatement");
                }
            }
            _ => panic!("Expected second statement to be a Let variant"),
        }

        match &statements[2] {
            Statement::Print(p) => {
                if let PrintValue::Variable(ident) = &p.value {
                    assert_eq!(ident.to_string(), "test");
                } else {
                    panic!("Expected a variable identifier inside PrintStatement");
                }
            }
            _ => panic!("Expected third statement to be a Print variant"),
        }

        match &statements[3] {
            Statement::Exit(e) => {
                assert_eq!(e.value.base10_parse::<i32>().unwrap(), 1);
            }
            _ => panic!("Expected fourth statement to be an Exit variant"),
        }
    }

    #[test]
    #[should_panic(expected = "Failed to parse source: \n\"invalid_keyword 123;\"\nError: unexpected token")]
    fn test_parse_invalid_syntax() {
        // Enforce that illegal code or missing tokens explicitly fail
        assert_parse!("invalid_keyword 123;", Program);
    }
}