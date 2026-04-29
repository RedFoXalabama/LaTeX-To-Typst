------------------------ BOOK HEADER -----------------------
#set page(
  paper: "a4",
  // Margine interno (gutter) più ampio per la rilegatura
    margin: (inside: 3.5cm, outside: 2.5cm, y: 3cm),
    numbering: "1",
)
#set heading(numbering: "1.1")

// Comportamento \chapter per i libri
#show heading.where(level: 1): it => {
  // Forza l'inizio del capitolo sulla pagina DESTRA (dispari)
  pagebreak(to: "odd", weak: true)
  v(15%) // Inizia a circa il 15% dell'altezza pagina

  // Stile "Capitolo X" sopra il titolo
  let n = counter(heading).display()
  text(1.2em, gray)[Capitolo #n]
  parbreak()
  text(2em, weight: "bold", it.body)

  v(3em)
}

// Header dinamico (Numero pagina a destra se dispari, a sinistra se pari)
#set page(header: context {
  let page_num = counter(page).get().first()
  if calc.odd(page_num) {
    align(right)[#page_num]
  } else {
    align(left)[#page_num]
  }
})

#set math.equation(numbering: "(1)")
#set par(
  first-line-indent: 1em, // Rientro di un carattere
  leading: 0.65em,        // Interlinea
  spacing: 1.2em,         // Spazio tra paragrafi
)

#show link: underline // Impostiano lo stile dei link a sottolineato
// ----------------------- END BOOK HEADER -----------------------



#let title = [LaTeX To Typst]
#let author = "Gianfranco e Vito"
#set document(author: author)

#let date = datetime.today()
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
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

    #v(2em)
#align(center)[
#text(1.2em)[Part I]
#v(0.5em)
#text(2em, weight: "bold")[part 1]
]
#v(2em)

    #v(2em)
#align(center)[
#text(1.2em)[Part II]
#v(0.5em)
#text(2em, weight: "bold")[part 2]
]
#v(2em)

    /*ERROR: NOT-YET-IMPLEMENTED \chapter{chapter}*/
    = section

    == subsection

    === subsubsection

    ==== paragraph

    ===== subparagraph



