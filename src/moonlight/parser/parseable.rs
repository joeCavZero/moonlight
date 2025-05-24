use crate::moonlight::utils::*;
use super::ast::*;

pub trait Parseable {
    fn parse(&mut self, tokens: &mut Vec<PositionedToken>) -> Ast;
}

//impl Parseable for Moonlight {
//    fn parse(&mut self, tokens: &mut Vec<PositionedToken>) -> Ast{
//        
//    }
//}