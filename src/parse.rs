use ops::{And, Not, Xor, Or, Imply};
use axiom::{Axiom};
use tokenizer::{Tokenizer, TokenInfo, Token};
use exp::{Exp};
use regex::Regex;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
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
	instrs: Vec<Rc<Exp>>,

	/// Tokens to parse
	tokens: Vec<Token<TokenType>>,

	/// The stack for generating the abstract syntax tree
	stack: VecDeque<Rc<Exp>>
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

	// TODO save also the stack
	fn save_state(&mut self) -> usize {
		self.index
	}

	// TODO restore also the stack
	fn restore_state(&mut self, restore: bool, old_state: usize) {
		if !restore {
			self.index = old_state;
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

	fn pop_stack_two(&mut self) -> (Rc<Exp>, Rc<Exp>) {
		if self.stack.len() <= 1 {
		    panic!("parse error: stack is too short");
		}
		(self.stack.pop_front().unwrap(), self.stack.pop_front().unwrap())
	}

	///////////////////////////////////////////////////////////////////// RULES

	fn rule_axiom(&mut self) -> bool {
		let old_state = self.save_state();
		let to_return =	self.tok_is_type(TokenType::Axiom);
		if to_return {
			let mut axiom_letter = self.tokens[self.index - 1].get_content();
			let axiom_letter = axiom_letter.chars().next().unwrap();
			self.stack.push_front(Axiom::new(axiom_letter));
		}
		self.restore_state(to_return, old_state);
		to_return
	}

	fn rule_not(&mut self) -> bool {
		let old_state = self.save_state();
		// TODO replace tok_is_type by a rule_value ?
		let to_return =	self.tok_is_type(TokenType::Not) &&
				self.rule_axiom();
		if to_return {
			if self.stack.len() == 0 {
			    println!("parse error, stack is empty");
			}
		    let val = self.stack.pop_front().unwrap();
		    self.stack.push_front(Not::new(val));
		}
		self.restore_state(to_return, old_state);
		to_return
	}

	fn rule_parenthesis(&mut self) -> bool {
		let old_state = self.save_state();
		let to_return =	self.tok_is_type(TokenType::OpenParenthesis) &&
				self.rule_expr() &&
				self.tok_is_type(TokenType::CloseParenthesis);
		self.restore_state(to_return, old_state);
		to_return
	}

	fn rule_value(&mut self) -> bool {
		let old_state = self.save_state();
		let to_return =	self.rule_not() ||
				self.rule_axiom() ||
				self.rule_parenthesis();
		self.restore_state(to_return, old_state);
		to_return
	}

	fn rule_and(&mut self) -> bool {
		let old_state = self.save_state();
		let to_return =	self.tok_is_type(TokenType::And) &&
				self.rule_value();
		if to_return {
		    let (rg, lf) = self.pop_stack_two();
		    self.stack.push_front(And::new(lf, rg));
		}
		self.restore_state(to_return, old_state);
		to_return
	}

	fn rule_or(&mut self) -> bool {
		let old_state = self.save_state();
		let to_return =	self.tok_is_type(TokenType::Or) &&
				self.rule_value();
		if to_return {
		    let (rg, lf) = self.pop_stack_two();
		    self.stack.push_front(Or::new(lf, rg));
		}
		self.restore_state(to_return, old_state);
		to_return
	}

	fn rule_xor(&mut self) -> bool {
		let old_state = self.save_state();
		let to_return =	self.tok_is_type(TokenType::Xor) &&
				self.rule_value();
		if to_return {
		    let (rg, lf) = self.pop_stack_two();
		    self.stack.push_front(Xor::new(lf, rg));
		}
		self.restore_state(to_return, old_state);
		to_return
	}

	fn rule_expr(&mut self) -> bool {
		let old_state = self.save_state();
		let mut to_return =	true;
		to_return = self.rule_value();
		let mut carry_on = to_return;
		while to_return && carry_on {
			carry_on = self.rule_and() || self.rule_or() || self.rule_xor();
		}
		self.restore_state(to_return, old_state);
		to_return
	}

	fn rule_instruction(&mut self) -> bool {
		let old_state = self.save_state();
		let to_return = self.rule_expr() &&
				self.tok_is_type(TokenType::Implies) &&
				self.rule_expr() &&
				Parser::optional(self.tok_is_type(TokenType::Comment)) &&
				self.tok_is_type(TokenType::EndLine);
		if to_return {
		    let (rg, lf) = self.pop_stack_two();
		    self.instrs.push(Imply::new(lf, rg));
		    self.stack.clear();
		}
		self.restore_state(to_return, old_state);
		to_return
	}

	fn rule_empty_line(&mut self) -> bool {
		let old_state = self.save_state();
		let to_return =
				Parser::optional(self.tok_is_type(TokenType::Comment)) &&
				self.tok_is_type(TokenType::EndLine);
		self.restore_state(to_return, old_state);
		to_return
	}

	/// Parse the string into an equation and reduce it.
	pub fn parse(to_parse: &String) -> Option<Vec<Rc<Exp>>>
	{
		// init parser struct
		let mut tokens = Parser::split_into_tokens(to_parse);
		tokens.push(Token::new(TokenType::EndLine, "\n".to_string()));
		println!("{:?}", tokens);
		let mut parser = Parser{
			index: 0,
			instrs: Vec::new(),
			tokens: tokens,
			stack: VecDeque::new()
		};

		// test tokens against rules
		let mut carry_on = true;
		while carry_on && !parser.reached_end() {
			carry_on = parser.rule_empty_line() || parser.rule_instruction();
		}

		// return value
		if carry_on {
		    Some(parser.instrs)
		} else {
		    None
		}
	}
}
