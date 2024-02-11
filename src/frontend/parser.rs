
use crate::frontend::lexer::{Lexer};
use crate::frontend::lexer::token::{Token, TokenType};
use crate::ast::attrs::{AstAttrs, AttrHandler, ExternalLinkage};
use crate::reports::{CompileError, Error, ErrorInfo, Reports};

pub struct Parser {
    tokens: Vec<Token>,
    token_index: usize,
    token: Token,
    reports: Reports,
}

macro_rules! report {
    ($self:ident, $error_type:expr) => { {
        $self.reports.add_error(CompileError::new($error_type, $self.token.get_location()));
        return Err(());
    }
    };
    ($self:ident, $error_type:expr, $info:expr) => {{
        $self.reports.add_error(CompileError::new($error_type, $self.token.get_location()).with_info($info));
        return Err(());
    }
    }
}

impl Parser {
    pub fn new(l: &Lexer) -> Parser {
        Parser {
            tokens: l.get_tokens().clone(),
            token_index: 0,
            token: l.get_tokens()[0].clone(),
            reports: Reports::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), ()> {
        self.parse_global(TokenType::EOF)
    }

    pub fn assert_global_item_next(&mut self, after: String) -> Result<(), ()> {
        match self.token.get_type() {
            TokenType::Fn |
            TokenType::Struct |
            TokenType::Enum |
            TokenType::Class |
            TokenType::Const |
            TokenType::Public |
            TokenType::Private |
            TokenType::Static |
            TokenType::Inline |
            TokenType::External |
            TokenType::Abstract |
            TokenType::Final |
            TokenType::Interface => Ok(()),
            _ => report!(self, Error::ExpectedItem("global item".to_string(), after), ErrorInfo {
                help: Some("There are only a few items that can be declared at the global scope".to_string()),
                see: Some("https://snowball-lang.gitbook.io/docs/language-reference/global-scope".to_string()),
                ..Default::default()
            }),
        }
    }

    pub fn parse_global(&mut self, terminator: TokenType) -> Result<(), ()> {
        let mut attrs = AttrHandler::new();
        while *self.token.get_type() != terminator {
            match self.token.get_type() {
                TokenType::EOF => report!(self, Error::UnexpectedEOF),
                TokenType::Public => {
                    self.next();
                    attrs.add_attr(AstAttrs::Privacy(true));
                    self.assert_global_item_next("public".to_string())?;
                } 
                TokenType::Private => {
                    self.next();
                    attrs.add_attr(AstAttrs::Privacy(false));
                    self.assert_global_item_next("private".to_string())?;
                }
                TokenType::Static => {
                    self.next();
                    attrs.add_attr(AstAttrs::Static);
                    self.assert_global_item_next("static".to_string())?;
                }
                TokenType::Inline => {
                    self.next();
                    attrs.add_attr(AstAttrs::Inline);
                    self.assert_global_item_next("inline".to_string())?;
                }
                TokenType::External => {
                    self.next();
                    match self.token.get_type() {
                        TokenType::String(data) => {
                            match data.as_str() {
                                "C" => attrs.add_attr(AstAttrs::External(ExternalLinkage::C)),
                                "snowball" => attrs.add_attr(AstAttrs::External(ExternalLinkage::Snowball)),
                                "system" => attrs.add_attr(AstAttrs::External(ExternalLinkage::System)),
                                _ => report!(self, Error::InvalidExternalSpecifier(data.clone()), ErrorInfo {
                                    help: Some("The external specifier must be one of the following: 'C', 'snowball', 'system'".to_string()),
                                    info: Some("Not a valid external specifier!".to_string()),
                                    note: Some("External specifiers are used to specify the data that is being imported from an external source".to_string()),
                                    see: Some("https://snowball-lang.gitbook.io/docs/language-reference/external-specifier".to_string()),
                                    ..Default::default()
                                }),
                            }
                            self.next();
                        }
                        _ => report!(self, Error::ExpectedItem("external specifier".to_string(), "external".to_string()), ErrorInfo {
                            help: Some("The external specifier must be a string literal".to_string()),
                            note: Some("External specifiers are used to specify the data that is being imported from an external source".to_string()),
                            see: Some("https://snowball-lang.gitbook.io/docs/language-reference/external-specifier".to_string()),
                            ..Default::default()
                        }),
                    }
                    self.assert_global_item_next("external".to_string())?;
                }
                TokenType::Abstract => {
                    self.next();
                    attrs.add_attr(AstAttrs::Abstract);
                    self.assert_global_item_next("abstract".to_string())?;
                }
                TokenType::Final => {
                    self.next();
                    attrs.add_attr(AstAttrs::Final);
                    self.assert_global_item_next("final".to_string())?;
                }
                _ => report!(self, Error::UnexpectedToken(self.token.value())),
            }
        }
        Ok(())
    }
    
    pub fn next(&mut self) {
        self.token_index += 1;
        self.token = self.tokens[self.token_index].clone();
    }

    pub fn get_reports(&self) -> &Reports {
        &self.reports
    }
}
