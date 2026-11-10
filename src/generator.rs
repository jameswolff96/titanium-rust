use std::{any::type_name_of_val, collections::HashMap};

use crate::parser::{ExitStatement, LetStatement, PrintStatement, PrintValue, Program, Statement};

#[derive(Default)]
pub struct SymbolTable {
    variables: HashMap<String, String>,
}

pub struct Generator {
    symbol_table: SymbolTable,
    output: String,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::default(),
            output: String::new(),
        }
    }

    pub fn generate_program(&mut self, program: Program) -> Result<String, String> {
        self.output.clear();

        self.output.push_str("// --- Generated Target Code ---\n");

        self.output.push_str("fn main() {\n");

        for stmt in program.0 {
            self.generate_statement(stmt)?;
        }

        self.output.push_str("}\n");

        Ok(self.output.clone())
    }

    fn generate_statement(&mut self, stmt: Statement) -> Result<(), String> {
        match stmt {
            Statement::Exit(exit_statement) => self.gen_exit(exit_statement),
            Statement::Let(let_statement) => self.gen_let(let_statement),
            Statement::Print(print_statement) => self.gen_print(print_statement),
        }
    }

    fn gen_exit(&mut self, stmt: ExitStatement) -> Result<(), String> {
        let code = stmt.value.base10_digits();
        self.output.push_str(&format!("    std::process::exit({});\n", code));
        Ok(())
    }

    fn gen_let(&mut self, stmt: LetStatement) -> Result<(), String> {
        let var_name = stmt.name.to_string();

        let val_str = self.lit_to_string(&stmt.value)?;

        self.symbol_table.variables.insert(var_name.clone(), val_str.clone());

        self.output.push_str(&format!("    let {} = {};\n", var_name, val_str));

        Ok(())
    }

    fn gen_print(&mut self, stmt: PrintStatement) -> Result<(), String> {
        match &stmt.value {
            PrintValue::Literal(lit) => {
                let val_str = self.lit_to_string(lit)?;
                self.output.push_str(&format!("    println!(\"{{}}\", {});\n", val_str));
            }
            PrintValue::Variable(ident) => {
                let var_name = ident.to_string();

                if self.symbol_table.variables.contains_key(&var_name) {
                    self.output.push_str(&format!("    println!(\"{{}}\", {});\n", var_name));
                } else {
                    return Err(format!("Compile Error: Reference to undeclared variable '{}'", var_name));
                }
            }
        }

        Ok(())
    }

    fn lit_to_string(&self, lit: &syn::Lit) -> Result<String, String> {
        match lit {
            syn::Lit::Int(lit) => Ok(lit.base10_digits().to_string()),
            syn::Lit::Str(lit) => Ok(format!("\"{}\"", lit.value())),
            syn::Lit::Bool(lit) => Ok(lit.value.to_string()),
            _ => return Err(format!("Generator Error: Unsupported literal type {}", type_name_of_val(&lit)))
        }
    }
}

pub fn generate(program: Program) -> Result<String, String> {
    let mut g = Generator::new();
    g.generate_program(program)
}
