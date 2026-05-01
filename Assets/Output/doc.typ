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


/* usepackage{graphicx} */ // Required for inserting images
/* usepackage{hyperref} */
/* usepackage{listings} */
/*WARN: NotImplemented("usepackage"): usepackage \usepackage{comment}*/

#let title = [LaTeX2Typst Documentation]
#let author = "Gianfranco Baccarella e Vito Di Bari"
#set document(author: author)
#let date = datetime(day: 1, month: 5, year: 2026)
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

= Introduzione


    LaTeX rappresenta un potente linguaggio di markup attualmente utilizzato in numerosi contesti, soprattutto accademici.
    La sua potenza rappresentativa permette di realizzare di documenti di testo secondo regole di formattazione e sintassi ben definite.

    LaTeX è definito come un linguaggio di markup di tipo WYSIWYM (What You See Is What You Mean), ossia all'utente è richiesto di redigere un file sorgente che sarà successivamente interpretato da un compilatore dedicato per produrre il documento finale.
    Questo approccio risultare può risultare troppo complesso per chi si approccia per la prima volta ad un nuovo strumento per la formattazione di testi differente dai classici programmi come Word di Microsoft.

    Typst è un nuovo linguaggio di mark-up che presenta una sintassi in generale più semplice rispetto a quella di LaTeX e che vanta di una serie di funzionalità built-in, ossia disponibili senza la necessità di importare package specifici, come le immagini o i blocchi di codice.

    Da questo nasce l'idea di creare un compilatore che sia in grado di tradurre un file sorgente scritto in linguaggio LaTeX nel corrispettivo scritto in linguaggio Typst. Il compilatore è stato modellato secondo un'architettura a due fasi:

-  back-end: trasforma il file sorgente scritto in LaTeX in una _rappresentazione intermedia_ (IR), basandosi sulle regole di sintassi proprie di LaTeX;
-  front-end: trasforma la IR nel file Typst.

    In questo caso specifico, la IR è stata implementata attraverso un Abstract Syntax Tree.

== Lavori correlati

==== #link("https://github.com/scipenai/tylax")[Tylax]


    Realizzato dall'utente #link("https://github.com/scipenai")[scipenai]. Si tratta di un progetto analogo, che permette di tradurre file LaTex in Typst e viceversa. Si tratta di progetto ancora in fase sviluppo, ma che si trova in una fase più avanzata rispetto a quello proposto da questo lavoro e che permette già di tradurre molte strutture sintattiche, compresa una sezione separata esclusiva per la matematica.

    Tuttavia, al momento, non supporta la traduzione di alcuni comandi che sono stati gestiti dal presente lavoro, come: `maketitle` per la realizzazione del titolo e alcuni blocchi, l'environment `comment` per creare blocchi di commenti e `flush` per l'allineamento del testo.

    È possibile provare una demo del software navigando verso la #link("https://convert.silkyai.cn")[pagina dedicata].

= Scope del traduttore


    Prima di passare allo sviluppo vero è proprio, è stato definito un documento che delinea lo lo scope del traduttore, ossia tutti i costrutti/comandi/environent che saranno supportati in questa prima versione dello strumento LaTeX2Typst.

    La lista delle feature supportate è contenuta in Tabella 1 ed è basata sull'elenco _LaTeX essentials_ riportato nella #link("https://www.overleaf.com/learn")[documentazione ufficiale di Overleaf].

/*WARN: EnvironmentBlockNotImplemented("table"): Environment block 'table' not implemented or wrong*/

/*WARN: WrongCommandOrNotImplemented("caption"): caption \caption{Tabella di compatibilità delle feature (LaTeX #sym.arrow.r Typst)}*/

#table(
	columns: 2,
	[*Feature*],
	[*Status*],
	[Struttura Documento],
	[Supportato],
	[Metadati e Titoli],
	[Supportato],
	[Sezionamento],
	[Supportato],
	[Formattazione Testo],
	[Supportato],
	[Simboli Speciali],
	[Supportato],
	[Allineamento],
	[Supportato],
	[Liste ed Elenchi],
	[Supportato],
	[Matematica],
	[Supportato],
	[Codice],
	[Supportato],
	[Tabelle],
	[Supportato],
	[Figure],
	[Supportato],
	[Link],
	[Supportato],
	[Spazi e Interruzioni],
	[Supportato],
	[Pacchetti],
	[Supportato],
	[Commenti],
	[Supportato],
)



= Struttura del progetto


    Il software è stato scritto interamente in linguaggio Rust.
    L'architettura del transpiler segue una pipeline di elaborazione divisa in tre fasi principali, riportate in figura ed analizzate nei paragrafi seguenti.

    Nell'ottica di rendere il software quanto più robusto possibile, si è posta particolare attenzione alla validazione dell'input per ogni fase dell'architettura.
    L'approccio è stato quello di continuare la traduzione in casi di errore ben definiti e circoscritti (es: commentando un comando LaTeX non supportato), altrimenti il processo viene arrestato con un errore quanto più chiaro possibile (es: il nome specificato nell'apertura di un environment LaTeX non combacia con quello in chiusura).

#context figure(
  image("Latex2Typst.png", width: 100%),
  caption: [struttura progetto],
) <fig-placeholder>


== Analisi Sintattica - Parsing


    Durante questo processo, il parser si assicura che il file in input, scritto in linguaggio LaTeX (source), abbia una struttura conforme a una grammatica ben specifica.
    Il parser non è stato scritto a mano, bensì generato automaticamente dalla libreria #link("https://docs.rs/pest/2.8.6/pest/")[Pest], un parser generator per Rust, sulla base della grammatica non ricorsiva a sinistra specificata nei file:

- `/src/latex parser/latex.pest`
- `/src/latex parser/latex math.pest`

    L'aspetto formale di Pest verrà approfondito nel paragrafo seguente.

    È riportato di seguito un frammento significativo della grammatica, dove è presente lo _start symbol_`file` della grammatica prevista.

```
file = { SOI ~ item* ~ EOI }
item = { block | command | text | newlines | linebreak | comment }
text = { (!("\\" | NEWLINE | comment) ~ ANY)+ }
    ```

    L'ordine di definizione delle regole è fondamentale in quanto il parser si fermerà al primo token non terminale che viene rispettato dal lessema analizzato, quindi per esempio: se si considera un lessema che rispetta sia la regola _command_, sia la regola _text_, il parser lo risolverà con la regola _command_ poiché presente prima nel pattern.

    L'output del parser sarà un ParseTree, ossia un albero che si origina da una singola radice, ordinato, e dove ogni nodo corrisponde a un match tra un lessema del file sorgente e token definito in grammatica.
    Ogni nodo rispetta la struttura definita dalla struttura `Pair`, i cui attributi sono:


- `rule`: identificatore della regola grammaticale rispettata
- `span`:
	- `str`: coordinate esatte del lessame inteso come sottostringa all'interno dell'intero file sorgente rappresentato come un'unica superstringa
	- `range`: il range occupato dai caratteri della sottostringa
- `inner`: lista ordinata di elementi figli, quindi altre istanze di `Pair`

    L'elenco delle regole grammaticali definite per il parsing del testo è riportato di seguito:


- `file`: intero testo del file di input
- `item`: componente generale del testo
- `name`: nome del comando
- `block`: blocco begin e end
- `command`: comando generale
- `linebreak`: interruzione di riga segnalata in LaTeX come doppio backslash
- `newlines`: interruzione di riga e nuove righe
- `comment`: commento monoriga
- `text`: testo semplice
- `required arg`: argomento obbligatorio del comando
- `argument`: contenuto del `required arg`
- `arg item`: tipologia di argomento di `argument`; può essere un altro comando o un testo, permettendo così il nesting dei comandi
- `arg text`: testo semplice di un `argument`
- `optional arg`: argomento opzionale del comando
- `optional list`: lista di argomenti che possono comporre un argomento opzionale
- `opt entry`: argomento singolo della lista
- `kv pair`: argomento chiave-valore
- `key`: chiave del `kv pair`
- `value`: valore del `kv pair`, può essere un semplice testo o una lista
- `simple value`: valore composto da testo semplice
- `value list`: valore composto da una lista di sottovalori
- `sub value list`: sottovalori della `value list`
- `opt item`: argomenti opzionali differenti dalla `kv pair`
- `opt text`: testo semplice dell' `opt item`
- `forbidden char`: insieme di caratteri da non considerare in determinate regole

=== Pest


    Pest è una libreria che genera parser di tipo _Recursive Descent Parser_ sulla base di grammatiche della famiglia _Parsing Expression Grammars_ (PEG).

    Le PEG sono grammatiche non ricorsive a sinistra che eliminano l'ambiguità alla radice grazie all'operatore di scelta ordinata: a differenza delle grammatiche tradizionali, il parser tenta di far corrispondere le espressioni seguendo rigorosamente l'ordine di definizione e selezionando la prima che ha successo. Questo approccio rende il processo di parsing diverso da quello usato per le _context-free grammars_, poiché la scelta di una produzione esclude automaticamente le successive invece che considerarle tutte di pari priorità.

    Le PEG, come #link("https://pdos.csail.mit.edu/~baford/packrat/thesis/thesis.pdf")[formulate originariamente] da Brian Frod, adottano il principio di _packrat parsing_ e backtracking arbitrario.

    Il packrat parsing è un algoritmo che punta ad ottimizzare le prestazioni del parser: l'idea è ridurre i tempi di parsing a costo di un utilizzo più aggressivo di memoria.
    In un parser tradizionale, se una regola fallisce dopo molti tentativi, il parser fa backtracking e c'è il rischio di ricalcolare la stessa regola sullo stesso pezzo di testo più volte, peggiorando le prestazioni con tempi potenzialmente esponenziali. Il packrat risolve il problema utilizzando una _tabella di memorizzazione_, cioè una struttura di dati in RAM che viene interrogata dal parser prima di risolvere una qualsiasi regola: se il risultato per il lessema corrente è presente (successo o fallimento), lo restituisce istantaneamente, altrimenti il parser lo risolve normalmente e salva il risultato in tabella.

    Un approccio del genere era improponibile quando i compilatori disponevano di poca RAM, richiedendo forte ottimizzazione sull'utilizzo di memoria.
    Con la disponibilità di memoria odierna, il packrat parsing è sicuramente applicabile e permette di ottenere prestazioni lineari temporali (O(n)), a fronte di un consumo di memoria maggiore e proporzionale alla dimensione del file.

    Pest dunque elimina l'ambiguità della grammatica con l'operatore di scelta ordinata indicato con `|`.
    Citando la #link("https://pest.rs/book/grammars/peg.html#non-backtracking")[documentazione di Pest]: "in un sistema con backtracking, si tornerebbe indietro di un passo, “non mangiando” un carattere, e poi si riproverebbe. Ma i PEG non fanno questo. Nella regola `first - second`, una volta che `first` viene analizzato con successo, ha consumato alcuni caratteri che non torneranno mai indietro. `second` può essere eseguito solo sull'input che `first` non ha consumato."
    Il parsing attutato tramite pest può essere riassunto cosi: prova a far rispettare la prima espressione, se ha successo continua continua a far rispettare il pattern finché non si conclude, se non ha successo prova con un'altra espressione. Se nessuna espressione ha successo allora genera errore.

    Tuttavia, a differenza dell'approccio originale, Pest non implementa il backtracking, quindi, citando ancora la documentazione ufficiale, durante l'analisi un'espressione può avere successo o fallisce direttamente, risultando in un errore bloccante per il software.

=== Errori sintattici


    Affinché il processo di traduzione sia robusto, il sorgente LaTeX dev'essere ben formato e rispettare la grammatica.
    Se il parser incontra un lessema che non riesce a risolvere, l'intero processo viene bloccato e viene prodotto un messaggio di errore in console che fa riferimento alla porzione di input che lo ha generato.

    Alcuni esempi di errori sintattici:

-  parentesi graffe di un argomento incomplete, solo aperta o solo chiusa
-  solo apertura o sola chiusura di un environment
-  apertura e chiusura di due environment differenti
-  caratteri numerici nella definizione di un environment

    Nella fase di parsing viene effettuato il controllo dei soli _errori sintattici_ (o _errori grammaticali_), ossia la sola correttezza sintattica della sequenza di lessemi rispetto alle regole formali definite nella grammatica PEG.
    Errori come comandi non riconosciuti oppure argomenti errati sono detti _errori semantici_ e saranno identificati più avanti nel processo di traduzione, dove si hanno più informazioni di contesto sull'intero file sorgente.

== Analisi Semantica - AST


    Si tratta di uno stadio più avanzato dell'analisi, dove è possibile interpretare le istruzioni scritte in LaTeX e verificarne la loro correttezza in base al contesto in cui si trovano, andando oltre la mera grammatica.

    Per rendere più agevole la fase di analisi semantica, il ParseTree ottenuto precedentemente viene sintetizzato in un _Abstract Syntax Tree_ (AST).
    L'AST è dunque il formato individuato per costruire la _rappresentazione intermedia_ del file sorgente.
    A differenza del ParseTree, l'AST è molto più sintetico, espressivo e leggibile, seppur mantenendo lo stesso contenuto informativo del file originario.

    Dire che l'AST sia più espressivo del ParseTree sta a significare che i nodi inglobano più informazione al loro interno, che può essere di carattere locale (es: informazioni derivati dal fatto che un certo nodo è individuato all'interno in un ambiente LaTeX) o globale (es: informazioni relative al tipo di documento definito a monte del file sorgente).

    Il formato finale di un nodo dell'AST lo si ottiene a valle di un processo di creazione e decorazione dello stesso con informazioni ereditate da nodi genitori o da nodi limitrofi. A titolo di esempio, la rappresentazione di un nodo che modella il comando `\\ textit` è effettuato come segue: viene definita un'instanza di CommandNode e decorata con un vettore `required args` che contiene il contenuto testuale da formattare in corsivo, identificato come un argomento obbligatorio. Nel ParseTree, invece, vengono creati molti più nodi annidati che riprendono tutte le regole grammaticali incontrate dal lessema, quindi il comando, l'identificativo del comando, il suo contenuto, il contenuto testuale, e così via, risultando molto più verboso.


- `CommandNode`
	- `name: String`: il nome del comando
	- `optional args: Vec OptionalArgNode `: gli argomenti opzionali di un comando, racchiusi tra parentesi quadre
	- `required args: Vec RequiredArgNode `: gli argomenti obbligatori di un comando, racchiusi tra parentesi graffe, in questo caso contiene un solo elemento
		- `TextNode`: contiene il valore testuale

    L'AST è dunque un albero basato su un insieme di strutture ed enumerativi definiti nel file `ast structure`. Ogni nodo sarà caratterizzato opportunamente dal tipo di struttura che più si addice, in base alla regola grammaticale d'origine.

    Nella definizione della struttura dell'AST sono stati definiti 15 tipi di nodi (strutture ed enumerativi):


- `AstDocument`: nodo radice - struct
- `AstItemNode`: nodo generale per una regola - enum
- `BlockNode`: blocco begin/end - struct
- `TextNode`: testo semplice - struct
- `WhitespaceNode`: spazi bianchi che precedono testo - struct
- `NewLinesNode`: numero di nuove righe - struct
- `LinebreakNode`: numero di interuzzione di riga - struct
- `CommentNode`: commenti mono riga - struct
- `CommandNode`: comandi latex - struct
- `RequiredArgNode`: argomenti richiesti dai comandi nelle parentesi graffe - struct
- `ArgItemNode`: oggetto presente nell'argomento richiesto - enum
- `OptionalArgNode`: argomenti opzionali dei comandi nelle parentesi quadre - struct
- `OptionalEntryNode`: oggetto presente nell'argomento opzionale - enum
- `KvPairNode`: argomento definito da una chiave ed un valore - struct
- `OptValueNode`: oggetto presente come valore nel KvPairNode - enum
- `OptItemNode`: oggetto presente nell'argomento opzionale - enum
- `MathNode`: nodo contente la matematica - struct

=== Errori semantici


    Come accennato in precedenza, in questa fase del processo è possibile effettuare controlli di validità dell'input (quindi dell'AST) più complessi e più sensibili al contesto, come: la verifica che un certo comando sia supportato oppure la verifica di un numero sufficiente di argomenti richiesti da uno specifico comando, ecc. Per realizzare quest'ultimo controllo, è stata realizzata un'ulteriore hashmap che ha come chiave l'identificativo del comando e come valore il numero minimo degli argomenti obbligatori attesi. Di seguito il frammento di codice responsabile del controllo.

```
if let Some(expected) = reqarg_count(&name) {
    if required_args.len() < expected as usize {
        warn!("Comando \\{}: expected {} required arguments, 
            found {}", name, expected, required_args.len());
            
        return Err(SemanticError::MissingRequiredArgItems);
    }
}
    ```

    Per le sopracitate tipologie di errore, viene stampato un warning a console, insieme ad un errore di tipo `UnsupportedCommand` o `MissingRequiredArgItems`, rispettivamente. Si tratta di una scelta fatta in fase di progettazione del software seguendo l'approccio adottato da LaTeX stesso: il compilatore cerca sempre di procedere nel rendering, a volte con forti assunzioni, nel tentativo di produrre comunque un output. Crediamo che si tratti di un approccio valido finché si rimane nel contesto dei linguaggi di markup dove il rischio più elevato è quello di avere un documento non propriamente fedele, a differenza del caso dei linguaggi di programmazione dove si rischia di alterare gravemente il flusso di un algoritmo.

    Sono stati definiti i seguenti errori semantici, come enumerativi. Questi sono stampati in console in caso di errore durante la fare di generazione dell'AST:


- `MissingFileNode`
- `MissingItemChild`
- `MissingCommandName`
- `MissingBlockName`
- `MissingRequiredArgItems`
- `MissingOptionalEntryItems`
- `MissingKeyInKvPair`
- `MissingValueInKvPair`
- `EmptyTextValue`
- `EmptyCommentValue`
- `InvalidNewlineCount`
- `InvalidLinebreakValue`
- `UnexpectedItemRule(Rule)`
- `UnexpectedArgItemRule(Rule)`
- `UnexpectedOptItemRule(Rule)`
- `UnexpectedOptionalEntryRule(Rule)`
- `UnsupportedCommand(String)`
- `UnsupportedEnvironment(String)`
- `MissingArgsForCommand(String, usize, usize)`
- `TextBeforeDocument`
- `UnexpectedRule(Rule)`
- `UnexpectedMathRule(Rule)`

== Generazione del codice Typst - CodeGen


    Si tratta dell'ultima fase della traduzione, dove avviene il _rendering_ dell'AST: consiste nel percorrere l'albero in modo ordinato per costruire il codice di output. La traduzione avviene elaborando i dati di ciascun nodo: il tipo del nodo determina la struttura sintattica da generare, mentre i suoi attributi forniscono il contenuto testuale e i parametri necessari.

    Il tipo di nodo che si vuole elaborare determina la funzione di rendering da richiamare. Tale funzione integra tutta la logica necessaria a recuperare le informazioni dal nodo stesso e dai suoi eventuali attributi, per poi produrre il codice Typst corrispondente.

    Un esempio semplice riguarda il rendering del nodo `TextNode`: il risultato della funzione di rendering è una semplice stringa da cui vengono rimosse le parentesi graffe. LaTeX non renderizza le parentesi graffe contenute nel testo nel file pdf finale, differentemente da Typst in cui invece compaiono.

```
pub(crate) fn render_text(text_node: &TextNode) -> String {
    text_node
        .value
        .chars()
        .filter(|c| *c != '{' && *c != '}')
        .collect()
}
    ```

    Differentemente, si è adottato un approccio più strutturato per situazioni più complesse.
    L'esempio principale è dato dalla gestione di ambienti e comandi LaTeX, dove è stata usata una hashmap, denominata `TransMap`, la cui chiave è l'identificativo (stringa) del commando e il valore è la specifica funzione di rendering, che presenta la seguente firma:


- `name`: stringa contente il nome del comando
- `reqs`: vettore di RequiredArgNode
- `opts`: vettore di OptionalArgNode
- `items`: vettore di AstItemNode. È presente solo nelle funzioni della TransMap dedicata agli ambienti per poter passare anche tutto ciò che è contenuto tra apertura e chiusura dell'ambiente stesso, per poter essere renderizzato di conseguenza.

    Le due hashmap, essendo usate in modo molto simile, sono state costruite implementando il _trait_`TransMap`. Lo si può considerare come il corrispettivo dell'_interfaccia_ in un contesto object-oriented.

```
pub trait TransMap<T> {
    fn translate(node: &T) -> Option<String>;
}
    ```

=== Funzionalità e Environment Implementati


    Chiaramente a causa del gran numero di funzioni presenti in LaTeX, per il progetto ci siamo posti l'obiettivo di implementare solamente quelle fondamentali, di uso comune, in modo da avere un software funzionante sui documenti più semplici.
    Le funzionalità implementate sono:

- `Text Formatting`: bold, italic, underline, color
- `Text Alignment`: centering, raggedright/left, flushright/left
- `Space and Breaks`: newline, break, hfill/vfill, pagebreak, newpage, clearpage
- `Package`: ragged2e, verbatim, hyperref, listings, graphicx 
	-  questi pacchetti non essendo necessari in typst, poiché coperti da funzioni built-in, sono semplicemente renderizzati come commenti.
- `Section`: section, subsection, subsubsection, paragraph, subparagraph, title, author, date, today, maketitle, tableofcontents
- `Hyperlinks`: href
- `Symbol`: textbackslash, textrightarrow, textleftarrow, \_, \{, \}

    Invece gli environment implementati sono:

- `Alignment`: center, flushright/left
- `Table`: tabular
- `Comment`: commenti multilinea
- `Text Listing`: liste itemize e enumerate, description
- `Graphics`: figure
- `Code`: verbatim, lstlisting

=== Errori codegen


    I due tipi di errori identificati in quest'ultima fase sono:

-  controllo su comandi e ambienti la cui logica di traduzione non è ancora implementata o l'identificativo del comando è stato scritto erroneamente
-  controllo su comandi e ambienti il cui corrispettivo in Typst non è stato trovato (al netto di eventuali estensioni)
    In entrambi i casi verrà stampato un warning su console, la porzione di documento impattata viene commentata per agevolare il successivo debugging e la codegen prosegue.

    Di seguito, viene riportato il frammento di codice responsabile del rendering dei comandi.
    Se la TransMap contiene una entry la cui chiave corrisponde al nome del comando, viene restituita la rendering associata e ci si aspetta l'output renderizzato.
    Altrimenti, l'errore viene gestito come già descritto.

```
pub(crate) fn render_command(command_node: &CommandNode) -> String {
    if let Some(rendered) = CommandTransMap::translate(command_node) {
        rendered
    } else {
        let out = String::new();
        drop_command_warn(
            COMMANDWARNING::WrongCommandOrNotImplemented(
                command_node.name.clone()),
            Option::from(out),
            Option::from(&*command_node.name),
            Option::from(command_node.required_args.clone()),
        )
    }
}
    ```

    In questa fase dell'architettura, il frontend è il componente responsabile della gestione degli errori, infatti essi dipendono anche dal linguaggio target.

= Test e Risultati


    Per verificare il corretto funzionamento del software sono stati definiti dei test d'integrazione automatici.
    Il codice sorgente relativo ai test si trova sotto nella directory `tests/cases` e si dividono in due tipologie:

- `oks`: sono stati testati diversi comandi e ambienti dove ci si aspetta che la traduzione vada a buon fine. L'asserzione finale prevede il confronto tra l'output effettivo e l'output atteso, salvato su un file separato nel percorso `tests/assertions/oks`;
- `errors`: ci si aspetta che la traduzione si concluda con un errore o con un file di output che presenta determinati commenti. Anche in quest'ultimo caso, l'asserzione finale prevede il confronto tra l'output effettivo e l'output atteso, salvato su un file separato nel percorso `tests/assertions/errors`;

    In aggiunta, per semplificare le operazioni di debugging, viene salvato l'output effettivo dei test nel percorso `tests/results`.

    È possibile eseguire tutti i test con il comando da terminale: `cargo test`.

== Esempio test case ok - text formatting


    Il test verifica la capacità del transpiler di convertire un testo LaTeX formattato in grassetto, in corsivo, sottolineato e colorato nel corrispettivo in Typst.
    I file usati dal test sono i seguenti.

    Input in linguaggio Latex:
```
\begin{document}
\textbf{Bold}
\textit{italic}
\underline{underline}

\textcolor{blue}{This is \textbf{bold and blue}.}
\end{document}
    ```

    Output atteso in linguaggio Typst:
```
*Bold*
_italic_
#underline[underline]

#text(blue)[This is *bold and blue*.]
    ```

== Esempio test case error - unknown command


    Il test verifica che il transpiler sia resiliente nel caso di errori dovuti a comandi inesistenti.
    I file usati dal test sono i seguenti.

    Input Latex:
```
\begin{document}
\commandoInesistente{argomento inesistente}
\end{document}
    ```
    Output Typst:
```
/*WARN: WrongCommandOrNotImplemented("commandoInesistente"): 
commandoInesistente \commandoInesistente{argomento inesistente}*/
    ```

== Esempio test case error - unmatched end


    Invece un esempio di input che genera un errore che blocca l'esecuzione é:
    Input Latex:
```
\begin{document}
    Contenuto
\end{article}
    ```

    Output sul terminale:
```
----------------------- PARSE ERROR -----------------------
Errore di parsing LaTeX alla riga 9, colonna 1
\end{article}
^
Atteso: item
-------------------------
    ```

= Esecuzione transpiler


    I seguenti passaggi illustrano come eseguire la traduzione di uno o più file LaTeX con estensione .tex nei corrispettivi file Typst .typ:

+  aprire il terminale ed eseguire il clone della repository ``` git clone https://github.com/RedFoXalabama/LaTeX-To-Typst ```
+  entrare nella root di progetto ``` cd LaTeX-To-Typst ```
+  installare le dipendenze di Rust ``` cargo build ```
+  posizionare il file da tradurre nella directory `Assets/Input/`
+  eseguire il software ``` cargo run ```

    Di default, saranno stampati solo i log di livello warning ed error.
    Per un logging più verboso, impostare la seguente variabile d'ambiente:
``` export RUST_LOG=info ```

= Conclusioni


    Il presente lavoro ha documentato la progettazione e l'implementazione di LaTeX2Typst, un transpiler scritto in linguaggio Rust capace di convertire documenti LaTeX in sorgenti Typst attraverso un'architettura modulare a tre fasi.

    Rispetto a soluzioni esistenti come Tylax, questo lavoro ha introdotto con successo il supporto a comandi aggiuntivi specifici quali: `maketitle`, l'allineamento tramite flush e la gestione dell'ambiente comment.

    Il software è stato realizzato con l'idea di essere resiliente agli errori: l'approccio adottato permette di arrestare il processo solo davanti a errori sintattici bloccanti, mentre gestisce i limiti semantici attraverso un sistema di warning e commenti nel file di output, preservando la continuità della traduzione.

== Sviluppi futuri


    Il progetto pone solide basi per evoluzioni successive che potrebbero trasformare il prototipo in uno strumento di produzione completo.
    I punti principali includono:


- *utilizzo di Rust*: l'architettura potrebbe beneficiare di un uso più esteso di pattern avanzati del linguaggio;
- *miglioramento funzionamento attuale*: si potrebbero rafforzare ed estendere alcune funzioni già implementate come le tabelle e la matematica;
- *copertura dei comandi e ambienti*: lo scope attuale, seppur funzionale per documenti di base, non copre l'immensa varietà di pacchetti di terze parti dell'ecosistema LaTeX;
- *gestione del contesto globale*: implementare un sistema per l'analisi del preambolo che permetta di mappare impostazioni globali (es. margini, font) direttamente nei file Typst.
- *GUI*: creazione di un'interfaccia grafica in modo da avere un'esperienza utente più gradevole e pratica per i meno esperti.

    In definitiva, LaTeX2Typst può rappresentare uno strumento per chi desidera
    trasporre i propri documenti verso un formato più modero ed user-friendly Typst.

== Note


    Il presente documento funge da dimostrazione pratica delle funzionalità implementate: è stato scritto impiegando costrutti LaTeX basilari, comandi ed environment inclusi nella tabella di compatibilità del software. Ciò garantisce che l'intero report possa essere processato dal sistema LaTeX2Typst, fornendo una prova immediata dell'efficacia della pipeline di traduzione.

