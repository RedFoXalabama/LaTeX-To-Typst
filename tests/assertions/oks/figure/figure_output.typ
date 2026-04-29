
#context figure(
  image("Latex2Typst.png", width: 80%),
  placement: none,
  caption: [caption test],
) <fig-placeholder>

// 1) linewidth (caso base)
#context figure(
  image("Latex2Typst.png", width: 50%),
  placement: top,
  caption: [test width=0.5/linewidth],
) <fig-test-linewidth>

// 2) textwidth
#context figure(
  image("Latex2Typst.png", width: 80%),
  placement: bottom,
  caption: [test width=0.8/textwidth],
) <fig-test-textwidth>

// 3) columnwidth
#context figure(
  image("Latex2Typst.png", width: 100%),
  placement: none,
  caption: [test width=columnwidth],
) <fig-test-columnwidth>

// 4) paperwidth
#context figure(
  image("Latex2Typst.png", width: 0.25 * page.width),
  placement: none,
  caption: [test width=0.25/paperwidth],
) <fig-test-paperwidth>

// 5) paperheight
#context figure(
  image("Latex2Typst.png", width: 0.2 * page.height),
  caption: [test width=0.2/paperheight],
) <fig-test-paperheight>

// 6) textheight
#context figure(
  image("Latex2Typst.png", width: 0.15 * page.height),
  caption: [test width=0.15/textheight],
) <fig-test-textheight>

// 7) columnsep
#context figure(
  image("Latex2Typst.png", width: 1em),
  caption: [test width=/columnsep],
) <fig-test-columnsep>

// 8) unitlength
#context figure(
  image("Latex2Typst.png", width: 2pt),
  caption: [test width=2/unitlength],
) <fig-test-unitlength>

// 9) pt
#context figure(
  image("Latex2Typst.png", width: 120pt),
  caption: [test width=120pt],
) <fig-test-pt>

// 10) mm
#context figure(
  image("Latex2Typst.png", width: 35mm),
  caption: [test width=35mm],
) <fig-test-mm>

// 11) cm
#context figure(
  image("Latex2Typst.png", width: 4cm),
  caption: [test width=4cm],
) <fig-test-cm>

// 12) in
#context figure(
  image("Latex2Typst.png", width: 2in),
  caption: [test width=2in],
) <fig-test-in>

// 13) em
#context figure(
  image("Latex2Typst.png", width: 18em),
  caption: [test width=18em],
) <fig-test-em>

// 14) ex
#context figure(
  image("Latex2Typst.png", width: 10em),
  caption: [test width=20ex],
) <fig-test-ex>
