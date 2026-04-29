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


#let title = [LaTeX To Typst]
#let author = "Gianfranco e Vito"
#set document(author: author)
#let date = datetime.today()
#set document(date: date)
#let date = datetime(day: 31, month: 12, year: 2026)
#set document(date: date)

/* usepackage{verbatim} */
/* usepackage{listing} */
/* usepackage{hyperref} */


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

prova-tra parentesi graffe-
testo pre commento // prova commento monolinea
/*
    Questo é un commento multilinea \textbf{aa}
*/

/*
    \begin{tabular}{ |c|c|csss| }
        \hline
        \textbf{cell1} & cell2 & cell3 \\
        \hline
        cell4 & cell5 & cell6 \\
        \hline
        cell7 & cell8 & cell9 \\
        \hline
    \end{tabular}
*/


*Bold*
_italic_
#underline[underline]

#set align(center);
#set align(left);
#set align(right);
#set align(right);
#set align(left);
/* usepackage{ragged2e} */
#align(center)[
center block *bold*
]
#align(left)[flushleft block ]
#align(right)[flushright block ]
#align(left)[FlushLeft block ]
#align(right)[FlushRight block ]

\
\
\
#h(1fr)
#v(1fr)
#pagebreak()
#pagebreak()
#pagebreak()


- [!]itemize 1
-
	+enumerate 1
	+
		-itemize 3
		-
			+enum 2
			+
				+enum 3
-itemize 2

/ description 1:description text 1

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


= section

== subsection

=== subsubsection

==== paragraph

===== subparagraph


#text(blue)[This is *bold and blue*.]

#link("https://www.overleaf.com/learn")
#link("https://www.overleaf.com/learn")[Overleaf Learn]// testo del link

/*WARN: NotImplemented("usepackage"): usepackage \usepackage{geometry}*/
/* usepackage{listings} */

/*WARN: EnvironmentBlockNotImplemented("tabularr"): Environment block 'tabularr' not implemented or wrong*/

*cell1*& cell2 & cell3 \

cell4 & cell5 & cell6 \

cell7 & cell8 & #table(
	columns: 3,
	[/*WARN: WrongCommandOrNotImplemented("hlinee"): hlinee \hlinee{}*/],
	[*cell1*],
	[cell2],
	[cell3],
	[cell4],
	[cell5],
	[cell6],
	[cell7],
	[cell8],
	[cell9],
)
\



```
    Text enclosed inside \texttt{verbatim} environment
    is printed directly
    and all \LaTeX{} commands are ignored.
```

```python
import numpy as np

def incmatrix(genl1,genl2):
    m = len(genl1)
    n = len(genl2)
    M = None #to become the incidence matrix
    VT = np.zeros((n*m,1), int)  #dummy variable

    #compute the bitwise xor matrix
    M1 = bitxormatrix(genl1)
    M2 = np.triu(bitxormatrix(genl2),1)

    for i in range(m-1):
        for j in range(i+1, m):
            [r,c] = np.where(M2 == M1[i,j])
            for k in range(len(r)):
                VT[(i)*n + r[k]] = 1;
                VT[(i)*n + c[k]] = 1;
                VT[(j)*n + r[k]] = 1;
                VT[(j)*n + c[k]] = 1;

                if M is None:
                    M = np.copy(VT)
                else:
                    M = np.concatenate((M, VT), 1)

                VT = np.zeros((n*m,1), int)

    return M
```

