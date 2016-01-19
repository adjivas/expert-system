use tokenizer::{Tokenizer, TokenInfo, Token};
use exp::{Exp};
use regex::Regex;

#[derive(PartialEq, Eq, Clone)]
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
	Unknow
}

pub struct Parser {
	/// The index of the first token which have to be parsed
	index: usize,

	/// Instructions parsed. This is the result of the parsing.
	instrs: Vec<Box<Exp>>,

	/// Tokens to parse
	tokens: Vec<Token<TokenType>>,

	/// This is the copy of the previous state to restore if nescessary
	/// using Parser::restore_state
	old_state: usize
}

impl Parser {

	fn split_into_tokens(to_parse: &String) -> Vec<Token<TokenType>>
	{
		let token_types = vec![
			TokenInfo::new(TokenType::OpenParenthesis, Regex::new(r"\(").unwrap()),
			TokenInfo::new(TokenType::CloseParenthesis, Regex::new(r"\)").unwrap()),
			TokenInfo::new(TokenType::Not, regex!("!")),
			TokenInfo::new(TokenType::And, Regex::new(r"\+").unwrap()),
			TokenInfo::new(TokenType::Or, Regex::new(r"\|").unwrap()),
			TokenInfo::new(TokenType::Xor, regex!(r"\^")),
			TokenInfo::new(TokenType::Implies, regex!("=>")),
			TokenInfo::new(TokenType::If, regex!("<=>")),
			TokenInfo::new(TokenType::EndLine, regex!("\n")),
			TokenInfo::new(TokenType::Axiom, regex!("[a-zA-Z]")),
			TokenInfo::new(TokenType::Comment, regex!("#.*")),
			TokenInfo::new(TokenType::Unknow, regex!(".*")),
		];
		let tokenizer = Tokenizer::new(token_types);
		tokenizer.split(to_parse)
	}

	fn reached_end(&self) -> bool {
		println!("{:?} < {:?}", self.index, self.tokens.len());
	    if self.index < self.tokens.len() {
	        false
	    } else {
	        true
	    }
	}

	fn save_state(&mut self) {
		self.old_state = self.index
	}

	fn restore_state(&mut self, restore: bool) {
		if !restore {
			self.index = self.old_state;
		}
	}

	/// If one rule is optional mark it Parser::optional(Rule)
	fn optional(_: bool) -> bool {
	    true
	}

	fn tok_is_type(&mut self, tok_type: TokenType) -> bool {
	    let found = self.tokens[self.index].get_type().clone() == tok_type;
	    if found {
	        self.index += 1;
	    }
	    found
	}

	fn rule_expr(&mut self) -> bool {
		self.save_state();
		let to_return =	self.tok_is_type(TokenType::Axiom);
		self.restore_state(to_return);
		to_return
	}

	fn rule_instruction(&mut self) -> bool {
		self.save_state();
		let to_return = self.rule_expr() &&
				self.tok_is_type(TokenType::Implies) &&
				self.rule_expr() &&
				Parser::optional(self.tok_is_type(TokenType::Comment)) &&
				self.tok_is_type(TokenType::EndLine);
		self.restore_state(to_return);
		to_return
	}

	fn rule_empty_line(&mut self) -> bool {
		self.save_state();
		let to_return =
				Parser::optional(self.tok_is_type(TokenType::Comment)) &&
				self.tok_is_type(TokenType::EndLine);
		self.restore_state(to_return);
		to_return
	}

	/// Parse the string into an equation and reduce it.
	pub fn parse(to_parse: &String) -> Option<Vec<Box<Exp>>>
	{
		// init parser struct
		let mut tokens = Parser::split_into_tokens(to_parse);
		tokens.push(Token::new(TokenType::EndLine, "".to_string()));
		let mut parser = Parser{
			index: 0,
			instrs: Vec::new(),
			old_state: 0,
			tokens: tokens
		};

		// test tokens against rules
		let mut carry_on = true;
		while carry_on && !parser.reached_end() {
			carry_on = parser.rule_empty_line() || parser.rule_instruction();
		}

		// return value
		println!("{:?} {:?}", carry_on, parser.reached_end());
		if carry_on {
		    Some(parser.instrs)
		} else {
		    None
		}
	}
}
