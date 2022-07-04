use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

#[derive(Debug, PartialEq)]
enum Token {
    VideoTitle,
    SectionTitle,
    CodeBlock,
    EditingNote,
    ScriptText,
    BlankLine,
}

enum CodeToken<'a> {
    Line,
    Comment,
    LineWithComment(&'a str, &'a str),
}

pub fn lex_and_split(some_file: &mut File) -> Result<()> {
    let mut lines = BufReader::new(some_file).lines();

    let mut script_file = File::create("script.txt")?;
    let mut code_file = File::create("code.txt")?;

    while let Some(line) = lines.next() {
        // ^^^ need to de-sugar for-in to not take ownership of the iterator
        if let Ok(ref l) = line {
            match classify_line(l) {
                Token::VideoTitle => {} // ignore video title
                Token::SectionTitle => {
                    script_file.write_all(b"---\n")?; // indicator to stop recording
                }
                Token::CodeBlock => {
                    handle_code_block(&mut lines, &mut script_file, &mut code_file)?;
                    writeln!(code_file)?;
                }
                Token::EditingNote => {} // ignore editing note
                Token::ScriptText => {
                    script_file.write_all(l.as_bytes())?;
                    writeln!(script_file)?;
                }
                Token::BlankLine => writeln!(script_file)?,
            }
        }
    }
    Ok(())
}

fn classify_line(line: &str) -> Token {
    if line.eq("") {
        Token::BlankLine
    } else if line.contains("```") {
        Token::CodeBlock
    } else if line.contains('>') {
        Token::EditingNote
    } else if line.contains("##") {
        Token::SectionTitle
    } else if line.contains('#') {
        Token::VideoTitle
    } else {
        Token::ScriptText
    }
}

fn handle_code_block<I>(lines: &mut I, script_file: &mut File, code_file: &mut File) -> Result<()>
where
    I: Iterator<Item = Result<String, std::io::Error>>,
{
    let mut code_line;
    loop {
        code_line = lines
            .next()
            .ok_or_else(|| anyhow::anyhow!("code block was never closed"))??;
        if code_line.contains("```") {
            code_file.write_all(b"---\n")?; // indicator of end of code block
            break;
        }
        match classify_code_line(&code_line) {
            CodeToken::Line => {
                code_file.write_all(code_line.as_bytes())?;
                writeln!(code_file)?;
            }
            CodeToken::Comment => {
                script_file.write_all(code_line.trim().as_bytes())?;
                writeln!(script_file)?;
                code_file.write_all(code_line.as_bytes())?;
                writeln!(code_file)?;
            }
            CodeToken::LineWithComment(line, comment) => {
                code_file.write_all(line.as_bytes())?;
                writeln!(code_file)?;
                script_file.write_all(comment.as_bytes())?;
                writeln!(script_file)?;
            }
        }
    }
    Ok(())
}

fn classify_code_line(line: &str) -> CodeToken {
    if line.trim().starts_with("//") {
        CodeToken::Comment
    } else if line.trim().contains("//") {
        let comment_starts_at = line.find("//").unwrap();
        // ^^^ this can't fail, so we're ok with this unwrap
        CodeToken::LineWithComment(line, &line[comment_starts_at..])
    } else {
        CodeToken::Line
    }
}
