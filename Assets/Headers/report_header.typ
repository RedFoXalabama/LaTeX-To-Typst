// ------------------------ REPORT HEADER -----------------------
#set page(
  paper: "a4",
  margin: (x: 3cm, y: 3cm),
  numbering: "1",       // Numerazione pagine semplice a fondo pagina
)

#set heading(numbering: "1.1")

#show heading.where(level: 1): it => {
  pagebreak(weak: true) // Forza il salto pagina (solo se necessario)
  v(4em)                // Spazio bianco in alto
  text(1.5em, strong(it))
  v(2em)
}

#set math.equation(numbering: "(1)")

#set par(
  first-line-indent: 1em, // Rientro di un carattere
  leading: 0.65em,        // Interlinea
  spacing: 1.2em,         // Spazio tra paragrafi
)

#show link: underline // Impostiano lo stile dei link a sottolineato
// ----------------------- END REPORT HEADER -----------------------