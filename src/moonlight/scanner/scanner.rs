use crate::moonlight::utils::*;

use super::positioned_token_vectorable::*;

fn read_file(path: &str) -> Result<String, String> {
    use std::fs;

    let raw = match fs::read_to_string(path) {
        Ok(content) => Ok(content.replace("\r", "")),
        Err(_) => Err(format!("The file {} does not exist or could not be read", path)),
    };

    raw

}

pub fn scan_tokens_from_file(file_path: &str, file_id: u32) -> Result<Vec<PositionedToken>, (String, Option<Position>)> {
    let raw = match read_file(file_path) {
        Ok(content) => content,
        Err(e) => return Err((e, None))
    };

    
    scan_string_and_generate_positioned_tokens(&raw, file_id)
    
} 



#[allow(unused_variables)]
fn scan_string_and_generate_positioned_tokens(source: &str, file_id: u32) -> Result<Vec<PositionedToken>, (String, Option<Position>)> {
    let mut tokens: Vec<PositionedToken> = Vec::new();
    let mut token_accumulator = String::new();
    let mut chars = source.chars().peekable();

    let mut actual_line = 1;
    let mut actual_column = 1;

    let mut initial_token_column = 1;

    let mut is_string_literal_mode = false;
    let mut is_commentary = false;
    let mut line_has_identation = false;

    while let Some(ch) = chars.next() {
        if token_accumulator.is_empty() {
            initial_token_column = actual_column;
        }
        match ch {
            '\t' => {
                if !is_commentary && !is_string_literal_mode {
                    line_has_identation = true;
                }
                actual_column += 1;
                continue;
            }
            '\n' => {
                // Finaliza comentário e reseta estados
                is_commentary = false;
                line_has_identation = false;
                
                // Adiciona token acumulado, se houver
                if !token_accumulator.is_empty() && !is_string_literal_mode {
                    match tokens.contexted_push(
                        token_accumulator.clone(),
                        file_id,
                        actual_line,
                        if !line_has_identation { Some(initial_token_column) } else { None },
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                        }
                    }
                    token_accumulator.clear();
                }

                actual_line += 1;
                actual_column = 1;
                continue;
            }
            '#' => {
                if !is_string_literal_mode {
                    // Inicia modo de comentário
                    is_commentary = true;
                    // Adiciona token acumulado, se houver
                    if !token_accumulator.is_empty() {
                        match tokens.contexted_push(
                            token_accumulator.clone(),
                            file_id,
                            actual_line,
                            if !line_has_identation { Some(initial_token_column) } else { None },
                        ) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                            }
                        }
                        token_accumulator.clear();
                    }
                } else {
                    // Dentro de string literal, '#' é tratado como caractere comum
                    token_accumulator.push(ch);
                }
                actual_column += 1;
                continue;
            }
            ' ' => {
                if is_commentary || is_string_literal_mode {
                    if is_string_literal_mode {
                        token_accumulator.push(ch);
                    }
                    actual_column += 1;
                    continue;
                }
                // Espaço fora de comentário ou string delimita um token
                if !token_accumulator.is_empty() {
                    match tokens.contexted_push(
                        token_accumulator.clone(),
                        file_id,
                        actual_line,
                        if !line_has_identation { Some(initial_token_column) } else { None },
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                        }
                    }
                    token_accumulator.clear();
                }
                actual_column += 1;
                continue;
            }
            '"' => {
                if is_commentary {
                    actual_column += 1;
                    continue;
                }
                if is_string_literal_mode {
                    // Fecha string literal
                    token_accumulator.push(ch);
                    match tokens.contexted_push(
                        token_accumulator.clone(),
                        file_id,
                        actual_line,
                        if !line_has_identation { Some(initial_token_column) } else { None },
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                        }
                    }
                    token_accumulator.clear();
                    is_string_literal_mode = false;
                } else {
                    // Inicia string literal
                    if !token_accumulator.is_empty() {
                        match tokens.contexted_push(
                            token_accumulator.clone(),
                            file_id,
                            actual_line,
                            if !line_has_identation { Some(initial_token_column) } else { None },
                        ) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                            }
                        }
                        token_accumulator.clear();
                    }
                    token_accumulator.push(ch);
                    is_string_literal_mode = true;
                }
                actual_column += 1;
                continue;
            }
            ',' | '[' | ']' => {
                if is_commentary || is_string_literal_mode {
                    if is_string_literal_mode {
                        token_accumulator.push(ch);
                    }
                    actual_column += 1;
                    continue;
                }
                if !token_accumulator.is_empty() {
                    match tokens.contexted_push(
                        token_accumulator.clone(),
                        file_id,
                        actual_line,
                        if !line_has_identation { Some(initial_token_column) } else { None },
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                        }
                    }
                    token_accumulator.clear();
                }
                match tokens.contexted_push(
                    ch.to_string(),
                    file_id,
                    actual_line,
                    if !line_has_identation { Some(initial_token_column) } else { None },
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                    }
                }
                actual_column += 1;
                continue;
            }
            ':' => {
                /*
                    This is made for recognize sticky label declarations
                    like <_Val:.word 1, 2, 3> as separated tokens.
                    Example:
                        <">_Val:.word 1, 2, 3> --> ["_Val:", ".word", "1", "2", "3"]
                 */
                if is_commentary || is_string_literal_mode {
                    if is_string_literal_mode {
                        token_accumulator.push(ch);
                    }
                    actual_column += 1;
                    continue;
                }

                token_accumulator.push(ch);

                if !token_accumulator.is_empty() {
                    match tokens.contexted_push(
                        token_accumulator.clone(),
                        file_id,
                        actual_line,
                        if !line_has_identation { Some(initial_token_column) } else { None },
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
                        }
                    }
                    token_accumulator.clear();
                }
                
            }
            _ => {
                if is_commentary {
                    actual_column += 1;
                    continue;
                }
                // Caractere genérico: acumula no token atual
                token_accumulator.push(ch);
                actual_column += 1;
            }
        }
    }

    // Adiciona o último token acumulado, se houver
    if !token_accumulator.is_empty() && !is_string_literal_mode {
        match tokens.contexted_push(
            token_accumulator.clone(),
            file_id,
            actual_line,
            if !line_has_identation { Some(initial_token_column) } else { None },
        ) {
            Ok(_) => {}
            Err(e) => {
                return Err((e, Some(Position::new(file_id, actual_line, Some(initial_token_column)))));
            }
        }
    }

    Ok(tokens)
}