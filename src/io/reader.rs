use crate::parser::parser::Parser;
use crate::rnodes::rnode::RNode;

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

pub struct RsonReader
{
    parser: RefCell<Parser>,
    pub file_path: Option<String>,
}

impl RsonReader
{
    pub fn from_file(path: &String) -> Self
    {
        let input = std::fs::read_to_string(&path).expect("Failed to read input file");
        Self { parser: RefCell::new(Parser::new_move(input)), file_path: Some(path.clone()) }
    }

    pub fn from_stdin() -> Self
    {
        let input = std::io::read_to_string(std::io::stdin()).expect("Failed to read from stdin");
        Self { parser: RefCell::new(Parser::new_move(input)), file_path: None }
    }

    pub fn parse(&self) -> Result<Rc<dyn RNode>, String>
    {
        let mut parser: RefMut<Parser> = self.parser.borrow_mut();
        parser.parse()
    }
}

