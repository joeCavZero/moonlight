use crate::moonlight::utils::*;

pub trait PositionedTokenVectorable {
    fn contexted_push(&mut self, token_string: String, file_id: u32, line: u32, column: Option<u32>) -> Result<(), String>;
}

impl PositionedTokenVectorable for Vec<PositionedToken> {
    fn contexted_push(&mut self, token_string: String, file_id: u32, line: u32, column: Option<u32>) -> Result<(), String> {
        
        let position = Position::new(file_id, line, column);

        match Token::from_string(&token_string) {
            Ok(token) => {
                let positioned_token = PositionedToken {
                    token,
                    position
                };
                self.push(positioned_token);
                Ok(())
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    
}