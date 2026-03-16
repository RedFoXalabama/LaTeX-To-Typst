use crate::latex_parser::Rule;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstDocument{
    pub items: Vec<AstItemNode>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstItemNode{
    Text(TextNode), // rule: text
    Newlines(NewlinesNode), // rule: newlines,
    Command(CommandNode) // rule: command
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextNode{
    pub value: String // rule: text
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewlinesNode {
    // utile se vuoi distinguere "\n" da "\n\n"
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandNode{
    pub name: String, // rule: name
    pub optional_args: Vec<OptionalArgNode>, // rule: optional_arg*
    pub required_args: Vec<RequiredArgNode>, // rule: required_arg*
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiredArgNode {
    pub items: Vec<ArgItemNode>, // rule: argument -> arg_item*
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgItemNode {
    Command(CommandNode),      // rule: command
    Group(RequiredArgNode),    // rule: required_arg annidato
    Newlines(NewlinesNode),      // rule: newlines
    Text(TextNode),            // rule: arg_text
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionalArgNode {
    pub entries: Vec<OptionalEntryNode>, // rule: optional_list? -> opt_entry*
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionalEntryNode {
    KeyValue(KvPairNode),      // rule: kv_pair
    Items(Vec<OptItemNode>),   // rule: opt_item+
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KvPairNode {
    pub key: String,
    pub value: OptValueNode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptValueNode {
    Simple(String), // rule: simple_value
    List(Vec<String>), // rule: value_list -> { sub_value_list (, sub_value_list)* }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptItemNode {
    Command(CommandNode),      // rule: command
    Group(RequiredArgNode),    // rule: required_arg
    Newline(NewlinesNode),      // rule: newlines
    Text(TextNode),            // rule: opt_text
}

// ERRORI PROPOSTI DA AI
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticError {
    MissingFileNode,
    MissingItemChild,
    MissingCommandName,
    MissingCommandArgument,
    MissingRequiredArgItems,
    MissingOptionalArgEntries,
    MissingOptionalEntryItems,
    MissingKeyInKvPair,
    MissingValueInKvPair,
    EmptyTextValue,
    InvalidNewlineCount,
    UnexpectedItemRule(Rule),
    UnexpectedArgItemRule(Rule),
    UnexpectedOptItemRule(Rule),
    UnexpectedOptionalEntryRule(Rule),
    UnexpectedRule(Rule),
}