use srt_linter::Lexer;
use srt_linter::Token;
use srt_linter::utils;

#[test]
fn test_token_generation_and_lexer_linting() {
    let test_file = "./tests/test.srt";
    let mut content: Vec<String> = vec![];
    if let Ok(lines) = utils::file::read_lines(test_file) {
        for line in lines.map_while(Result::ok) {
            content.push(line);
        }
    }

    assert_ne!(content.len(), 0);

    let lexer_result = Lexer::new(content, true, true);
    let mut lexer = match lexer_result {
        Ok(res) => res,
        Err(_) => panic!("Lexer has failed to initialize."),
    };
    let lexed_result = lexer.lex();
    let (tokens, issues) = match lexed_result {
        Ok(res) => res,
        Err(_) => panic!("Tokenization has failed."),
    };

    assert_eq!(issues, 0);
    assert_eq!(
        tokens,
        vec![
            Token::Count(1, 1),
            Token::StartTime(136612, 2),
            Token::EndTime(139376, 2),
            Token::Subtitle((
                vec![
                    "Senator, we're making".into(),
                    "our final approach into Coruscant.".into()
                ],
                3
            )),
            Token::Count(2, 6),
            Token::StartTime(139482, 7),
            Token::EndTime(141609, 7),
            Token::Subtitle((vec!["Very good, Lieutenant.".into()], 8)),
            Token::Count(3, 10),
            Token::StartTime(193336, 11),
            Token::EndTime(195167, 11),
            Token::Subtitle((vec!["We made it.".into()], 12)),
            Token::Count(4, 14),
            Token::StartTime(198608, 15),
            Token::EndTime(200371, 15),
            Token::Subtitle((vec!["I guess I was wrong.".into()], 16)),
            Token::Count(5, 18),
            Token::StartTime(200476, 19),
            Token::EndTime(202671, 19),
            Token::Subtitle((vec!["There was no danger at all.".into()], 20)),
            Token::Eof,
        ]
    );
}
