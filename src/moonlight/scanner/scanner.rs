use crate::moonlight::utils::*;

use super::traits::*;

fn read_file(path: &str) -> Result<String, String> {
    use std::fs;

    let raw = match fs::read_to_string(path) {
        Ok(content) => Ok(content.replace("\r", "")),
        Err(e) => Err(format!("Error reading file {}: {}", path, e))
    };

    raw

}

pub fn scan_tokens_from_file(file_path: &str, file_id: u32) -> Result<Vec<PositionedToken>, String> {
    let raw = match read_file(file_path) {
        Ok(content) => content,
        Err(e) => return Err(e)
    };

    Ok(
        scan_string_and_generate_positioned_tokens(&raw, file_id)
    )
} 



#[allow(unused_variables)]
fn scan_string_and_generate_positioned_tokens(source: &str, file_id: u32) -> Vec<PositionedToken> {
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
                    tokens.contexted_push(
                        token_accumulator.clone(),
                        file_id,
                        actual_line,
                        if !line_has_identation { Some(initial_token_column) } else { None },
                    );
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
                        tokens.contexted_push(
                            token_accumulator.clone(),
                            file_id,
                            actual_line,
                            if !line_has_identation { Some(initial_token_column) } else { None },
                        );
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
                    tokens.contexted_push(
                        token_accumulator.clone(),
                        file_id,
                        actual_line,
                        if !line_has_identation { Some(initial_token_column) } else { None },
                    );
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
                    tokens.contexted_push(
                        token_accumulator.clone(),
                        file_id,
                        actual_line,
                        if !line_has_identation { Some(initial_token_column) } else { None },
                    );
                    token_accumulator.clear();
                    is_string_literal_mode = false;
                } else {
                    // Inicia string literal
                    if !token_accumulator.is_empty() {
                        tokens.contexted_push(
                            token_accumulator.clone(),
                            file_id,
                            actual_line,
                            if !line_has_identation { Some(initial_token_column) } else { None },
                        );
                        token_accumulator.clear();
                    }
                    token_accumulator.push(ch);
                    is_string_literal_mode = true;
                }
                actual_column += 1;
                continue;
            }
            ',' => {
                if is_commentary || is_string_literal_mode {
                    if is_string_literal_mode {
                        token_accumulator.push(ch);
                    }
                    actual_column += 1;
                    continue;
                }
                // ',' é um delimitador em instruções
                if !token_accumulator.is_empty() {
                    tokens.contexted_push(
                        token_accumulator.clone(),
                        file_id,
                        actual_line,
                        if !line_has_identation { Some(initial_token_column) } else { None },
                    );
                    token_accumulator.clear();
                }
                tokens.contexted_push(
                    ch.to_string(),
                    file_id,
                    actual_line,
                    if !line_has_identation { Some(initial_token_column) } else { None },
                );
                actual_column += 1;
                continue;
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
        tokens.contexted_push(
            token_accumulator.clone(),
            file_id,
            actual_line,
            if !line_has_identation { Some(initial_token_column) } else { None },
        );
    }

    tokens
}