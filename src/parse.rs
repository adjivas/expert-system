use tokenizer::{Tokenizer, TokenInfo, Token};
use exp::{Exp};
use regex::Regex;

#[derive(PartialEq, Clone)]
pub enum TokenType
{
	OpenParenthesis,
	CloseParenthesis,
	Not,
	And,
	Or,
	Xor,
	Implies,
	If,
	Comment,
	EndLine,
	Axiom,
}

pub struct Parser {
	/// The index of the first token which have to be parsed
	index: i32,

}

impl Parser {
	fn split(to_parse: &String) -> Vec<Token<TokenType>>
	{
		let token_types = vec![
			TokenInfo::new(TokenType::OpenParenthesis, Regex::new(r"\(").unwrap()),
			TokenInfo::new(TokenType::CloseParenthesis, Regex::new(r"\)").unwrap()),
			TokenInfo::new(TokenType::Not, regex!("!")),
			TokenInfo::new(TokenType::And, Regex::new(r"\+").unwrap()),
			TokenInfo::new(TokenType::Or, Regex::new(r"\|").unwrap()),
			TokenInfo::new(TokenType::Xor, regex!("^")),
			TokenInfo::new(TokenType::Implies, regex!("=>")),
			TokenInfo::new(TokenType::If, regex!("<=>")),
			TokenInfo::new(TokenType::EndLine, regex!("\n")),
			TokenInfo::new(TokenType::Axiom, regex!("[a-zA-Z]")),
			TokenInfo::new(TokenType::Comment, regex!("#.*")),
		];
		let tokenizer = Tokenizer::new(token_types);
		tokenizer.split(to_parse)
	}

	fn rule_instruction(&mut self) -> bool {


		// let ret = !rule_
		true
	}

	fn rule_empty_line(&mut self) -> bool {
		true
	}

	fn rule_paradigm(&mut self) -> bool {
		true
	}

	/// Parse the string into an equation and reduce it.
	pub fn parse(to_parse: &String) -> Vec<Box<Exp>>
	{
		//split string into tokens
		let tokens = Parser::split(to_parse);
		Vec::new()
	}
}
