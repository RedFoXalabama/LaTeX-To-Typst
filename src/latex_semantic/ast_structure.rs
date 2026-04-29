use crate::latex_parser::Rule;

// NODO PADRE
// é composto da una vettore di AstItemNode che rappresentano i vari elementi del documento (testo, comandi, newlines)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstDocument {
    pub items: Vec<AstItemNode>,
}

// AstItemNode é il nodo più generale di tutti e può rappresentare un testo, un comando o una sequenza di newlines.
// Ogni variante dell'enum corrisponde a una regola specifica della grammatica (text, command, newlines).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstItemNode {
    Block(BlockNode),
    Text(TextNode),           // rule: text
    Newlines(NewlinesNode),   // rule: newlines,
    Linebreak(LinebreakNode), // rule: linebreak
    Command(CommandNode),     // rule: command
    Comment(CommentNode),     // rule: comment
    Whitespace(WhitespaceNode), // rule: whitespace
    RawText(TextNode),        // rule: text (raw)
}

// BlockNode rappresenta una qualsiasi porzione di codice Latex racchiuso tra i comandi \begin e \end.
// Contiene a sua volta un vettore generico di AstItemNode, e quindi testo, comandi o altri blocchi ovviamente.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockNode {
    pub name: String,                        // rule: name
    pub required_args: Vec<RequiredArgNode>, // rule: required_arg*
    pub optional_args: Vec<OptionalArgNode>, // rule: optional_arg*
    pub items: Vec<AstItemNode>,
}

// TextNode rappresenta del semplice testo da mostrare a schermo
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextNode {
    pub value: String, // rule: text
}

// WhitespaceNode rappresenta spazi e tabulazioni
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhitespaceNode {
    pub value: String, // rule: whitespace
}

// NewLineNode rappresenta tutti gli andare a capo, e contiene un contatore per contare le nuove linee
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewlinesNode {
    // utile se vuoi distinguere "\n" da "\n\n"
    pub count: usize,
}

// Linebreak raccoglie le \\ che vengono rappresentate con \\\\
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinebreakNode {
    pub value: String,
}

// COMMENT commenti mono riga preceduti da %
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentNode {
    pub value: String,
}

// CommandNode rappresenta i comandi di LaTeX, con il loro nome e i loro argomenti (opzionali e obbligatori).
// \comando[opt]{req}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandNode {
    pub name: String,                        // rule: name
    pub optional_args: Vec<OptionalArgNode>, // rule: optional_arg*
    pub required_args: Vec<RequiredArgNode>, // rule: required_arg*
}

// RequiredArgNode rappresenta l'insieme di nodi di argomento obbligatorio, che possono contenere a loro volta comandi, testo o newlines (annidamento).
// Composto quindi da 0 o più ArgItemNode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiredArgNode {
    pub items: Vec<ArgItemNode>, // rule: argument -> arg_item*
}

// ArgItemNode rappresenta ciò che può essere presente come argomento obbligatorio in un comando di Latex
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgItemNode {
    Command(CommandNode),     // rule: command
    Group(RequiredArgNode),   // rule: required_arg annidato
    Newlines(NewlinesNode),   // rule: newlines
    Linebreak(LinebreakNode), // rule: linebreak
    Text(TextNode),           // rule: arg_text
}

// OptionalArgNode rappresenta l'insieme di nodi di argomento opzionali,
// che possono contenere a loro volta coppie chiave-valore e altri elementi opzionali (comandi, testo, newlines).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionalArgNode {
    pub entries: Vec<OptionalEntryNode>, // rule: optional_list? -> opt_entry*
}

// OptionalEntryNode rappresenta ciò che può essere presente come elemento opzionale in un comando di Latex,
// che può essere una coppia chiave-valore (es. key=value) o altri elementi opzionali (comandi, testo, newlines).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionalEntryNode {
    KeyValue(KvPairNode),    // rule: kv_pair
    Items(Vec<OptItemNode>), // rule: opt_item+
}

// KvPairNode rappresenta una coppia chiave-valore,
// dove la chiave è una stringa e il valore può essere un semplice valore o una lista di valori (es. key=value1,value2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KvPairNode {
    pub key: String,
    pub value: OptValueNode,
}

// OptValueNode rappresenta il valore di una coppia chiave-valore,
// che può essere un semplice valore (stringa) o una lista di valori (es. value1,value2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptValueNode {
    Simple(String),    // rule: simple_value
    List(Vec<String>), // rule: value_list -> { sub_value_list (, sub_value_list)* }
}

// OptItemNode rappresenta ciò che può essere presente come elemento opzionale in un comando di Latex,
// che può essere un comando, un gruppo di argomento obbligatorio annidato, una sequenza di newlines o del semplice testo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptItemNode {
    Command(CommandNode),   // rule: command
    Group(RequiredArgNode), // rule: required_arg
    Newlines(NewlinesNode), // rule: newlines
    Text(TextNode),         // rule: opt_text
}

// ERRORI SEMANTICI
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticError {
    MissingFileNode,
    MissingItemChild,
    MissingCommandName,
    MissingBlockName,
    MissingRequiredArgItems,
    // MissingOptionalArgEntries, //in latex non sono un problema le [] vuote
    MissingOptionalEntryItems,
    MissingKeyInKvPair,
    MissingValueInKvPair,
    EmptyTextValue,
    EmptyCommentValue,
    InvalidNewlineCount,
    InvalidLinebreakValue,
    UnexpectedItemRule(Rule),
    UnexpectedArgItemRule(Rule),
    UnexpectedOptItemRule(Rule),
    UnexpectedOptionalEntryRule(Rule),
    UnsupportedCommand(String),
    UnsupportedEnvironment(String),
    MissingArgsForCommand(String, usize, usize),
    TextBeforeDocument,
    UnexpectedRule(Rule),
}
