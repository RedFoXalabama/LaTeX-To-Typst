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