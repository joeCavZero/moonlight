use crate::moonlight::utils::*;

pub trait PositionedTokenVectorTrait {
    fn contexted_push(&mut self, token_string: String, file_id: u32, line: u32, column: Option<u32>);
}

impl PositionedTokenVectorTrait for Vec<PositionedToken> {
    fn contexted_push(&mut self, token_string: String, file_id: u32, line: u32, column: Option<u32>) {
        let token = Token::from_string(&token_string);
        let position = Position::new(file_id, line, column);
        let positioned_token = PositionedToken {
            token,
            position
        };
        self.push(positioned_token);
    }
}

