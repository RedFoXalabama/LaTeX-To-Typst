// ------------------------ ARTICLE HEADER -----------------------
#set page(
  paper: "a4",
  margin: (x: 2.5cm, y: 3cm),
  numbering: "1",       // Numerazione pagine semplice a fondo pagina
  header: none,         // Solitamente senza header particolare
)

#set heading(numbering: "1.1")

#show heading.where(level: 1): it => {
  v(1.5em, weak: true)
  it
  v(1em, weak: true)
}

#set math.equation(numbering: "(1)")

#set par(
  first-line-indent: 1em, // Rientro di un carattere
  leading: 0.65em,        // Interlinea
  spacing: 1.2em,         // Spazio tra paragrafi
)

#show link: underline // Impostiano lo stile dei link a sottolineato
// ----------------------- END ARTICLE HEADER -----------------------


/*ERROR: NOT-YET-IMPLEMENTED \usepackage{graphicx}*/ // Required for inserting images
/* usepackage{hyperref} */
/* usepackage{listings} */

#let title = [LaTeX2Typst Documentation]
#let author = "Gianfranco Baccarella e Vito Di Bari"
#set document(author: author)
#let date = datetime(day: 13, month: 4, year: 2026)
#set document(date: date)



#set document(title: title)
#align(center)[
#text(3em, weight: "bold")[#title]
#v(0em)
#text(1.8em)[#author]
#v(0em)
#text(1.5em)[#date.display("[day] [month repr:long] [year]")]
]
#v(2em)
#outline()

/*ERROR: NOT-YET-IMPLEMENTED \chapter{Capitolo prova}*/
/*ERROR: WRONG-COMMAND OR NOT-IMPLEMENTED \agagag{fpa}*/

= Introduction


== Contesto dello sviluppo

LaTeX rappresenta un potente linguaggio di markup attualmente utilizzato in numerosi contesti accademici e non per la sua potenza rappresentativa di documenti di testo ordinati secondo regole di formattazione ben definite. Nonostante ciò la sua sintassi e semantica possono risultare complesse per dei neofiti che si approcciano per la prima volta ad un nuovo strumento per la formattazione di testi differente dai classici programmi come Word di Microsoft. Per questo esiste Typst un nuovo linguaggio di markup che ha l'obiettivo di ricoprire le stesse funzioni di LaTeX, ma con una sintassi più semplici e senza la necessità di implementare package per funzioni base come le immagini o i commenti.
Da questo nasce l'idea di creare un Compilatore che possa prendere in input un file di testo in linguaggio LaTeX e fornire in output un file in linguaggio Typst effettuando quindi una traduzione da un linguaggio di markup ad un altro.

== Stato dell'arte

Al momento della realizzazione del progetto é stata individuata una repository github con lo stesso obiettivo: convertire dell'input in linguagguio LaTeX in output Typst. La repository, denominata #link("https://github.com/scipenai/tylax")[Tylax] realizzata dall'utente scipenai, di cui é possibile provare il funzionamento al seguente #link("https://convert.silkyai.cn")[link], é ancora in corso di sviluppo, ma dai risultati pubblicati si nota uno sviluppo avanzato che permette già di tradurre molte strutture sintattiche, compresa una sezione separata esclusiva per la matematica. Nonostante ciò Tylax non supporta al momento la traduzione di differenti comandi come: "maketitle" per la realizzazione del titolo e  alcuni blocchi begin/end come "comment" per creare blocchi di commenti e "flush" per l'allineamento del testo. Inoltre le repository si propone di effettuare una traduzione inversa prendendo in input un file in linguaggio Typst e restituendo quindi un file in linguaggio LaTeX.

= Sturuttura del progetto

Il software é stato scritto in linguaggio RUST per poter sfruttare la potenzialità della sua libreria #link("https://docs.rs/pest/2.8.6/pest/")[Pest], un parser general purpose, con cui é stato possibile definire una grammatica di riferimento ed effettuare un parsing sul testo scritto secondo il markup LaTeX per ottenere un ParseTree, una rappresentazione ad albero della struttura sintattica dell'intero file, basata sulla grammatica da noi definita in modo da scomporre nelle varie componenti il testo.
La struttura del progetto é abbastanza lineare, prendiamo in input il file LaTeX ed ogni componente dà in input il proprio output alla successiva, i vari passaggi possono essere rappresentati cosi:
    
+ *Input*: File Latex + LaTeX Grammar ->    
+ Parser -> Parse Tree ->    
+ Semantic -> Abastract Syntatx Tree ->     
+ CodeGen -> *Output*: File Typst

== Analisi Sintattica - Parsing

Come già inizialmente definito, il software prende in input un file scritto in linguaggio LaTeX (input.tex) e per prima cosa quindi viene effettuato il parsing del testo in modo da poter ottenere il precedentemente citato ParseTree. Per effettuare ciò abbiamo deciso di utilizzare la libreria Pest di Rust, la cui ci ha fornito delle funzioni per potere generare un Parser che potesse seguire determinate regole grammaticali definite nel file *latex.pest*. La grammatica si basa su definite regole che la stringa dovrà rispettare fino a potersi identificare nella regola più "restrittiva" navigando verso il basso dell'albero, quindi possiamo immaginare anche le regola grammaticali come un albero rovesciato in cui in cima, alla root, vi é presente la regola più generale in questo caso *file*, che si dirama verso il basso definendo altre regole che possono essere composte dai loro nodi figli. Senza entrare nel dettaglio, in questo documento, della grammatica possiamo fare l'esempio iniziale in cui:

```
file = { SOI ~ item* ~ EOI }
item = { block | command | text | newlines | linebreak | comment }
text = { (!("\\" | NEWLINE | comment) ~ ANY)+ }
```

Come possiamo vedere un file può essere composto da zero o più di un item, la definizione delle regole segue delle espressioni molto simili alle regex, ed a sua volta l'item deve essere composto da uno solo dei diversi elementi definiti. Item definisce quindi una regola generale che identifica ogni elemento presente nel file, successivamente si entra nello specifico di questi elementi come nella regola text che definisce come testo qualsiasi elemento che contenga uno o più caratteri che non inizi con la backslash, altrimenti identificherebbe come comando, e che non rispetti la regola "NEWLINE" o "comment".
Dopo aver generato un Parser basato sulla nostra grammatica, il sofware effettua quindi il parsing del file di input restituendo in output un Parse Tree in cui suddividiamo ogni elemento del file nella rispettiva regola grammaticale ed organizzati come un albero in cui ogni elemento, definito come Pair, possiede:
    
- *rule*, la regola grammaticale rispettata    
- *span*, un oggetto di tipo Span, che contiene a sua volta         
	- *str*, la stringa di riferimento estratta        
	- *range*, il range occupato dai caratteri        
- *inner*, gli elementi figli

Qui inseriamo la lista delle regole grammaticali definite per il parsing del testo.
    
- *file*, intero testo del file di input    
- *item*, componente generale del testo    
- *name*, nome del comando    
- *block*, blocco begin e end    
- *command*, comando generale    
- *linebreak*, interruzione di riga segnalate in latex come doppio backslash    
- *newlines*, interruzione di riga e nuove righe    
- *comment*, commento mono riga    
- *text*, testo semplice    
- *required arg*, argomento obbligatorio del comando    
- *argument*, argomento del required arg    
- *arg item*, tipologia di argomento di argument, può essere un altro comando o un testo, permettendo cosi il nesting dei comandi    
- *arg text*, testo semplice di un argument    
- *optional arg*, argomento opzionale del comando    
- *optional list*, lista di argomenti che possono comporre un argomento opzionale    
- *opt entry*, argomento singolo della lista    
- *kv pair*, argomento chiave valore    
- *key*, chiave del kv pair    
- *value*, valore del kv pair può essere un semplice testo o una lista    
- *simple value*, valore composto da testo semplice    
- *value list*, valore composto da una lista di sottovalori    
- *sub value list*, sottovalori della value list    
- *opt item*, argomenti opzionali differenti dalla kv pair    
- *opt text*, testo semplice dell'opt item    
- *forbidden char*, insieme di caratteri da non considerare in determinate regole

== Analisi Semantica - AST

Dopo aver effettuato il parsing sul testo ed aver ottenuto un ParseTree che ci permette di suddividere il testo in componenti sintattici incapsulando la loro logica, possiamo passare ad un analisi semantica del testo prendendo in input il ParseTree ed ottenere in output una rappresentazione intermedia del codice attraverso l'Abstract Syntax Tree in cui rappresentiamo il testo in una nuova rappresentazione logica ad albero in cui possiamo semplificare la rappresentazione del ParseTree, che risulta essere troppo innestato a causa delle varie regole grammaticali, ottenendo cosi un albero meno folto accomunando differenti regole sotto un unico enumerativo in modo da ragrupparli in base alle funzioni di rendering da utilizzare durante la generazione del nuovo codice; ed inoltre per aggiungere o semplificare informazioni come ad esempio il numero di nuove righe che nel ParseTree vengono rappresentate semplicemente come testo "n" differentemente dall'AST in l'oggetto *NewLineNode* possiede un attributo *count*. Ad esempio la stringa *titleLaTeX To Typst* che nel ParseTree viene scomposta in 7 nodi ognuno padre dell'altro, differentemente nell'AST é rappresentato con solo 3 nodi.
    
- *CommandNode*, che possiede gli attributi         
	- *Name*, il nome del comando        
	- *OptionalArgs*, gli argomenti opzionali di un comando racchiusi tra parentesi quadre        
	- *RequiredArgs*, gli argomenti obbligatori di un comando racchiusi tra parentesi graffe, in questo caso contiene un solo *RequiredArgNode*, che a sua volta possiede un attributo items di tipo *Text*             
		- *TextNode*, contiene il valore testuale all'interno delle parentesi quadre.            

Se la precedente struttura ad albero era basata sulla grammatica, questa nuova struttura ad albero che andiamo a realizzare é basata su una lista di strutture ed enumerativi da noi definiti, nel file _ast structure_, che vengono assegnati ai vari nodi in base alle loro regole grammaticali d'origine. Rimanendo sull'esempio del precedente comando, vediamo la struttura con cui costruiamo un *ComandNode*:
```
pub struct CommandNode {
    pub name: String,
    pub optional_args: Vec<OptionalArgNode>,
    pub required_args: Vec<RequiredArgNode>,
}
```
Come si può notare la struttura ComandNode possiede i tre 3 attributi prima definiti; particolare da notare sono gli ultimi due attributi che sono dei vettori di altri tipi di strutture definiti per poter incapsulare le rispettive tipologie di argomenti.
Nella definizione della struttura dell'AST sono stati definiti 15 tra strutture ed enumerativi:
    
- *AstDocument*, nodo radice - struct    
- *AstItemNode*, nodo generale per una regola - enum    
- *BlockNode*, blocco begin/end - struct    
- *TextNode*, testo semplice - struct    
- *NewLinesNode*, numero di nuove righe - struct    
- *LinebreakNode*, numero di interuzzione di riga - struct    
- *CommentNode*, commenti mono riga - struct    
- *CommandNode*, comandi latex - struct    
- *RequiredArgNode*, argomenti richiesti dai comandi nelle parentesi graffe - struct    
- *ArgItemNode*, oggetto presente nell'argomento richiesto - enum    
- *OptionalArgNode*, argomenti opzionali dei comandi nelle parentesi quadre - struct    
- *OptionalEntryNode*, oggetto presente nell'argomento opzionale - enum    
- *KvPairNode*, argomento definito da una chiave ed un valore - struct    
- *OptValueNode*, oggetto presente come valore nel KvPairNode - enum    
- *OptItemNode*, oggetto presente nell'argomento opzionale - enum
Inoltre nella stessa struttura sono definiti degli errori Semantici come enumerativi, che vengono stampati dalla console in caso di errore durante la fare di generazione dell'AST: _MissingFileNode,    MissingItemChild, MissingCommandName, MissingBlockName, MissingRequiredArgItems, MissingOptionalArgEntries, MissingOptionalEntryItems, MissingKeyInKvPair, MissingValueInKvPair, EmptyTextValue, EmptyCommentValue, InvalidNewlineCount, InvalidLinebreakValue, UnexpectedItemRule(Rule), UnexpectedArgItemRule(Rule), UnexpectedOptItemRule(Rule), UnexpectedOptionalEntryRule(Rule), UnexpectedRule(Rule)._

In questa fase quindi passiamo al setaccio l'intero ParseTree foglia per foglia per ottenere alla fine una rappresentazione intermedia del codice che però non é ancora il nostro linguaggio di destinazione. L'AST rappresenta una via di mezzo tra il linguaggio di partenza e quello di destinazione che ci permette di effettuare una più facile e corretta traduzione, attraverso la componente di *CodeGen*, nel nuovo linguaggio, nel nostro caso Typst.

== Generazione del codice Typst - CodeGen

Come introdotto, l'ultima fase della traduzione é la generazione del nuovo linguaggio mantenendo la semantica invariata; vogliamo quindi lo stesso risultato da parte del nuovo linguaggio. Per fare ciò prendiamo in input l'Abstract Syntax Tree e tramite la navigazione in esso, foglia per foglia, generiamo un codice equivalente, effettuiamo un "rendering"; che nel nostro caso assume il significato di scrivere la traduzione nel file di output.
Il rendering del nuovo codice avviene iterando una funzione di mapping su ogni item dell'AstDocument raccogliendo i risultati che saranno mano mano aggiunti al file. Per ogni nodo foglia su cui iteriamo effettuiamo una funzione di mapping differente poiché bisogna gestire le differenti casistiche, questa scelta é effettuata in base al rispettivo enumerativo del nodo foglia. Un esempio semplice é il rendering del nodo TextNode il cui risultato della funzione di mappamento é una semplice stringa da cui vengono rimosse le parentesi graffe poiché in latex queste non vengono renderizzate nel file pdf differentemente da typst in cui invece compaiono nel documento finale.
```
pub(crate) fn render_text(text_node: &TextNode) -> String {
    text_node
        .value
        .chars()
        .filter(|c| *c != '{' && *c != '}')
        .collect()
}
```
Differentemente per situazioni più complesse come i blocchi ed i comandi é stata implementata un hashmap, denominata *TransMap*, in cui la chiave é rappresentata dal nome del commando e il valore la specifica funzione per trattarlo.


