mod ast_structure;
pub use ast_structure::AstDocument;
use crate::latex_semantic::ast_structure::*;

use pest::iterators::{Pair, Pairs};
use crate::latex_parser::Rule;

// build_ast
// └─ build_document        (file)
//      └─ build_item        (item → text | newline | command)
//          ├─ build_text
//          ├─ build_newline
//          └─ build_command (name + optional_arg* + required_arg*)
    //          ├─ build_optional_arg → build_opt_entry
    //          │   ├─ build_kv_pair → build_value
    //          │   └─ build_opt_item
    //          └─ build_required_arg → build_arg_item


pub fn build_ast(mut pairs: Pairs<Rule>) -> Result<AstDocument, SemanticError> {
    let file_pair = pairs.next().ok_or(SemanticError::MissingFileNode)?;
    if file_pair.as_rule() != Rule::file {
        return Err(SemanticError::UnexpectedRule(file_pair.as_rule()));
    }
    println!("BUILDIAMO IL FILE");

    build_document(file_pair)
}

// file = { SOI ~ item* ~ EOI }
fn build_document(file_pair: Pair<Rule>) -> Result<AstDocument, SemanticError> {
    let mut items = Vec::new();

    for child in file_pair.into_inner() {
        match child.as_rule() {
            Rule::item => items.push(build_item(child)?),
            Rule::EOI => {} // ignorato nell'AST
            other => return Err(SemanticError::UnexpectedRule(other)),
        }
    }

    Ok(AstDocument { items })
}

// item = { command | text | newlines }
fn build_item(pair: Pair<Rule>) -> Result<AstItemNode, SemanticError> {
    let child = pair.into_inner().next().ok_or(SemanticError::MissingItemChild)?;

    match child.as_rule() {
        Rule::text => Ok(AstItemNode::Text(build_text(child)?)),
        Rule::newlines => Ok(AstItemNode::Newlines(build_newlines(child)?)),
        Rule::command => Ok(AstItemNode::Command(build_command(child)?)),
        other => Err(SemanticError::UnexpectedRule(other)),
    }
}

// text = { (!("\\" | NEWLINE) ~ ANY)+ }
fn build_text(pair: Pair<Rule>) -> Result<TextNode, SemanticError> {
    let value = pair.as_str().to_string();
    if value.is_empty() {
        return Err(SemanticError::EmptyTextValue);
    }
    Ok(TextNode {value})
}

// newlines = { NEWLINE+ }
fn build_newlines(pair: Pair<Rule>) -> Result<NewlinesNode, SemanticError> {
    let count = pair.as_str().chars().filter(|&c| c == '\n').count();
    if count == 0 {
        return Err(SemanticError::InvalidNewlineCount);
    }
    Ok(NewlinesNode {count})
}

// command = { "\\" ~ name ~ optional_arg* ~ required_arg* }
fn build_command(pair: Pair<Rule>) -> Result<CommandNode, SemanticError> {
    // la dichiarazione di tipo Vec<?> dipende dalla definizione in ast_structure
    let mut name: Option<String> = None;
    let mut optional_args: Vec<OptionalArgNode> = Vec::new();
    let mut required_args: Vec<RequiredArgNode> = Vec::new();

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::name => name = Some(child.as_str().to_string()),
            Rule::optional_arg => optional_args.push(build_optional_arg(child)?), // optional_arg = { "[" ~ optional_list? ~ "]" }
            Rule::required_arg => required_args.push(build_required_arg(child)?), // required_arg = { "{" ~ argument ~ "}" }
            other => return Err(SemanticError::UnexpectedRule(other)),
        }
    }

    Ok(CommandNode {
        name: name.ok_or(SemanticError::MissingCommandName)?,
        optional_args,
        required_args
    })
}

// --------------------------------- REQUIRED ARG --------------------------------------------------
// required_arg = { "{" ~ argument ~ "}" }
fn build_required_arg(pair: Pair<Rule>) -> Result<RequiredArgNode, SemanticError> {
    let mut items: Vec<ArgItemNode> = Vec::new();

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::argument => { // argument = { arg_item* }
                for arg_item in child.into_inner() {
                    match arg_item.as_rule() {
                        Rule::arg_item => items.push(build_arg_item(arg_item)?),
                        other          => return Err(SemanticError::UnexpectedArgItemRule(other)),
                    }
                }
            }
            other => return Err(SemanticError::UnexpectedRule(other)),
        }
    }

    Ok(RequiredArgNode { items })
}

// arg_item = { command | required_arg | newline | arg_text }
fn build_arg_item(pair: Pair<Rule>) -> Result<ArgItemNode, SemanticError> {
    let child = pair.into_inner().next().ok_or(SemanticError::MissingItemChild)?;

    match child.as_rule() {
        Rule::command      => Ok(ArgItemNode::Command(build_command(child)?)),
        Rule::required_arg => Ok(ArgItemNode::Group(build_required_arg(child)?)),
        Rule::newlines      => Ok(ArgItemNode::Newlines(build_newlines(child)?)),
        Rule::arg_text     => Ok(ArgItemNode::Text(build_text(child)?)), //usiamo la stessa regola per buildare il testo
        other              => Err(SemanticError::UnexpectedArgItemRule(other)),
    }
}


// --------------------------------- OPTIONAL ARG --------------------------------------------------
// optional_arg  = { "[" ~ optional_list? ~ "]" }
fn build_optional_arg(pair: Pair<Rule>) -> Result<OptionalArgNode, SemanticError> {
    let mut entries = Vec::new();

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::optional_list => { // optional_list = { opt_entry ~ ("," ~ opt_entry)* }
                for entry in child.into_inner() {
                    match entry.as_rule() {
                        Rule::opt_entry => entries.push(build_opt_entry(entry)?),
                        other           => return Err(SemanticError::UnexpectedOptionalEntryRule(other)),
                    }
                }
            }
            other => return Err(SemanticError::UnexpectedRule(other)),
        }
    }

    Ok(OptionalArgNode { entries })
}

// opt_entry = { (WHITE_SPACE* ~ kv_pair ~ WHITE_SPACE*) | opt_item+ }
fn build_opt_entry(pair: Pair<Rule>) -> Result<OptionalEntryNode, SemanticError> {
    let first_rule = pair.clone().into_inner().next().map(|p| p.as_rule());

    match first_rule {
        Some(Rule::kv_pair) => {
            let kv = build_kv_pair(pair.into_inner().next().unwrap())?;
            Ok(OptionalEntryNode::KeyValue(kv))
        }
        Some(Rule::opt_item) => {
            let items: Result<Vec<OptItemNode>, SemanticError> = pair
                .into_inner()
                .map(build_opt_item)
                .collect();
            Ok(OptionalEntryNode::Items(items?))
        }
        Some(other) => Err(SemanticError::UnexpectedOptionalEntryRule(other)),
        None        => Err(SemanticError::MissingOptionalEntryItems),
    }
}

// kv_pair = { key ~ WHITE_SPACE* ~ "=" ~ WHITE_SPACE* ~ value }
fn build_kv_pair(pair: Pair<Rule>) -> Result<KvPairNode, SemanticError> {
    let mut key: Option<String> = None;
    let mut value: Option<OptValueNode> = None;

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::key   => key = Some(child.as_str().to_string()),
            Rule::value => value = Some(build_value(child)?),
            other       => return Err(SemanticError::UnexpectedRule(other)),
        }
    }

    Ok(KvPairNode {
        key:   key.ok_or(SemanticError::MissingKeyInKvPair)?,
        value: value.ok_or(SemanticError::MissingValueInKvPair)?,
    })
}

// value        = { value_list | simple_value }
fn build_value(pair: Pair<Rule>) -> Result<OptValueNode, SemanticError> {
    let child = pair.into_inner().next().ok_or(SemanticError::MissingValueInKvPair)?;

    match child.as_rule() {
        Rule::simple_value => Ok(OptValueNode::Simple(child.as_str().trim().to_string())), // simple_value = { (!( "," | "]" ) ~ ANY)+ }
        Rule::value_list   => { // value_list   = { "{" ~ sub_value_list ~ ("," ~ sub_value_list)* ~ "}" }
            let sub_values: Vec<String> = child
                .into_inner()
                .filter(|p| p.as_rule() == Rule::sub_value_list)
                .map(|p| p.as_str().trim().to_string())
                .collect();
            Ok(OptValueNode::List(sub_values))
        }
        other => Err(SemanticError::UnexpectedRule(other)),
    }
}

// opt_item = { command | required_arg | newline | opt_text }
fn build_opt_item(pair: Pair<Rule>) -> Result<OptItemNode, SemanticError> {
    let child = pair.into_inner().next().ok_or(SemanticError::MissingItemChild)?;

    match child.as_rule() {
        Rule::command      => Ok(OptItemNode::Command(build_command(child)?)),
        Rule::required_arg => Ok(OptItemNode::Group(build_required_arg(child)?)),
        Rule::newlines      => Ok(OptItemNode::Newline(build_newlines(child)?)),
        Rule::opt_text     => Ok(OptItemNode::Text(build_text(child)?)), // usiamo la stessa funzione per buildare tanto é sempre testo
        other              => Err(SemanticError::UnexpectedOptItemRule(other)),
    }
}