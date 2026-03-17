mod ast_structure;
pub use ast_structure::AstDocument;
use crate::latex_semantic::ast_structure::*; //importo tutte le strutture e gli enumerati che compongono l'AST

use pest::iterators::{Pair, Pairs};
use crate::latex_parser::Rule;

// ALBERO AST (Abstract Syntax Tree) - rappresentazione ad albero della struttura sintattica del documento LaTeX,
// costruita a partire dal parse tree di pest.
// L'AST è più astratto e semantico rispetto al parse tree, e viene utilizzato per analisi successive o trasformazioni.

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


// FUNZIONE PRINCIPALE PER LA COSTRUZIONE DELL'AST, CHE VERRÀ CHIAMATA DAL MAIN DOPO AVER OTTENUTO IL PARSE TREE DI PEST
// prende in input Pairs<Rule>, quindi un insieme su cui iterare di Pair<Rule>, che rappresentano i nodi del Parse Tree
// il nodo finale che viene restituito é un AstDocument una struttura composta da un Vec<AstItemNode>, che rappresenta l'AST del documento LaTeX
pub fn build_ast(mut pairs: Pairs<Rule>) -> Result<AstDocument, SemanticError> {
    let file_pair = pairs.next().ok_or(SemanticError::MissingFileNode)?;
    if file_pair.as_rule() != Rule::file {
        return Err(SemanticError::UnexpectedRule(file_pair.as_rule()));
    }
    println!("BUILDIAMO IL FILE");

    build_document(file_pair)
}

// Costruisce il documento partendo dal nodo file che contiene una serie di item (testo, comandi, newlines)
// ed é avvolto tra SOI ed EOI per definire i limiti del testo
// file = { SOI ~ item* ~ EOI }
fn build_document(file_pair: Pair<Rule>) -> Result<AstDocument, SemanticError> {
    let mut items: Vec<AstItemNode> = Vec::new();

    // iteriamo sui figli del nodo file, che possono essere item o EOI (End Of Input)
    for child in file_pair.into_inner() {
        match child.as_rule() {
            Rule::item => items.push(build_item(child)?),
            Rule::EOI => {} // ignorato nell'AST
            other => return Err(SemanticError::UnexpectedRule(other)),
        }
    }

    Ok(AstDocument { items })
}

// ----------------------------- ITEM = COMMAND | TEXT | NEWLINES ----------------------------------

// Un item può essere composto da un comando, un testo o una nuova linea a capo
// se l'elemento non corrisponde a nessuno di questi elementi, beh cacca addosso -> UnexpectedRule
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

// Un testo é qualsiasi cosa che non inizi con un \ di comando o una nuova linea ed é composto da qualsiasi carattere
// se il nodo é vuoto (anche se non dovrebbe esserlo, visto che la regola richiede almeno un carattere), allora cacca addosso -> EmptyTextValue
// text = { (!("\\" | NEWLINE) ~ ANY)+ }
fn build_text(pair: Pair<Rule>) -> Result<TextNode, SemanticError> {
    let value = pair.as_str().to_string();
    if value.is_empty() {
        return Err(SemanticError::EmptyTextValue);
    }
    Ok(TextNode {value})
}


// Una nuova linea é composta da uno o più caratteri di nuova linea (\n), e viene rappresentata da un nodo che conta quante nuove linee ci sono
// se il nodo é segnato come NEWLINE, ma non contiene "nuove linee", allora cacca addosso -> InvalidNewlineCount
// newlines = { NEWLINE+ }
fn build_newlines(pair: Pair<Rule>) -> Result<NewlinesNode, SemanticError> {
    let count = pair.as_str().chars().filter(|&c| c == '\n').count();
    if count == 0 {
        return Err(SemanticError::InvalidNewlineCount);
    }
    Ok(NewlinesNode {count})
}

// Una comando é composto da un nome (che segue il \) e da una serie di argomenti opzionali (racchiusi tra []) e argomenti obbligatori (racchiusi tra {})
// se il comando presenta elementi che non rispecchiano le Rule degli elementi, allora cacca addosso -> UnexpectedRule
// command = { "\\" ~ name ~ optional_arg* ~ required_arg* }
fn build_command(pair: Pair<Rule>) -> Result<CommandNode, SemanticError> {
    let mut name: Option<String> = None;
    let mut optional_args: Vec<OptionalArgNode> = Vec::new();
    let mut required_args: Vec<RequiredArgNode> = Vec::new();

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::name => name = Some(child.as_str().to_string()), // Some diventerà string
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
// il required argument é utilizzato anche per identificare ogni elemento racchiuso tra { }


// Un Required Argument é un argomento obbligatorio di un comando che può essere composto da un argument
// un argument é composto da arg_item multipli poiché possiamo avere comandi e testo insieme o anche un altro elemento rinchiuso in {esempio}
// se l'argument con contiene un oggetto che rispetta la regola di Arg_item, allora cacca addosso -> UnexpectedArgItemRule
// required_arg = { "{" ~ argument ~ "}" }
fn build_required_arg(pair: Pair<Rule>) -> Result<RequiredArgNode, SemanticError> {
    let mut items: Vec<ArgItemNode> = Vec::new();

    // iteriamo per estrarre ogni arg_item che può rappresentare differenti strutture di Nodi
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

// Un argument item é un oggetto che viene utilizzato come required argument in presenza multipla, quindi un required argument può essere composto
// da più di un argomento, ma non necessariamente uno poiché possiamo avere {vuoto}.
// Un argument item può essere un comando + un testo + un andare a capo + un nuovo required_arg:
// Ex: \textbf{\title[opt1]{Il gatto} era sul \n tavolo {} }
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

// Un argomento opzionale é rinchiuso tra [] e può contenere una optional list di opt_entry separati da virgole,
// che può essere una coppia chiave-valore o una serie di opt_item (comando, testo, andare a capo, required_arg)
// EX: \usepackage[letterpaper = true,top=2cm\textwidth, points={1, cane,3 ,4}]{geometry}
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

// Una opt_entry può essere una kv_pair separati da spazi oppure un opt_item che può essere composto da { command | required_arg | newline | opt_text }
// quindi possiamo avere una lista di coppie valori insieme ad un testo, opportunamente separati da una virgola
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

// Una KV PAIR é una coppia composta da una chiave ed un valore e serve per impostare delle proprietà nei comandi
// possono essere separate da spazi, ma non é obbligatorio, e la chiave e il valore devono essere separati da un uguale (=)
// la chiave é una stringa che non contiene spazi, mentre il valore può essere una stringa semplice o una lista di valori racchiusi tra {}
// ex: \usepackage[letterpaper = true,top=2cm\textwidth, points={1, cane,3 ,4}]{geometry}
// kv_pair = { key ~ WHITE_SPACE* ~ "=" ~ WHITE_SPACE* ~ value }
fn build_kv_pair(pair: Pair<Rule>) -> Result<KvPairNode, SemanticError> {
    let mut key: Option<String> = None;
    let mut value: Option<OptValueNode> = None;

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::key   => key = Some(child.as_str().to_string()), // strina
            Rule::value => value = Some(build_value(child)?), // stringa semplice o lista di valori tra {,}
            other       => return Err(SemanticError::UnexpectedRule(other)),
        }
    }

    Ok(KvPairNode {
        key:   key.ok_or(SemanticError::MissingKeyInKvPair)?,
        value: value.ok_or(SemanticError::MissingValueInKvPair)?,
    })
}

// Il vaore di una chiave che può essere una stringa semplice (che non contiene virgole o parentesi)
// oppure una value_list composta da sub_value_list racchiusi tra {} e separati da virgole
// ex: \usepackage[letterpaper = true,top=2cm\textwidth, points={1, cane,3 ,4}]{geometry}
// value = { value_list | simple_value }
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

// Un optional item é un nodo presente solo negli optional argument e può essere composto da una serie di comandi, {}, \n o opt_text
// opt_text, é differente da arg_text, infatti opt_text = { (!("\\" | "[" | "]" | NEWLINE) ~ ANY)+ }, quindi non può iniziare con altre [] o nuove linee
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