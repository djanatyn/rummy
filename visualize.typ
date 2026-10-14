#import "@preview/deckz:0.3.1"

#let data = (
  starting_hand: ("7D", "JC", "9D", "8H", "JH", "8C", "QC", "5D", "10C", "2S", "8S", "8D", "4C"),
  found: (
    (
      sequence: ("7D", "8D"),
      rest: ("8H", "5D", "9D", "2S", "8S", "4C", "8C", "10C", "JC", "QC")
    ),
    (
      sequence: ("8D", "9D"),
      rest: ("8H", "5D", "7D", "2S", "8S", "4C", "8C", "10C", "JC", "QC")
    ),
    (
      sequence: ("10C", "JC"),
      rest: ("8H", "5D", "7D", "8D", "9D", "2S", "8S", "4C", "8C", "QC")
    ),
    (
      sequence: ("JC", "QC"),
      rest: ("8H", "5D", "7D", "8D", "9D", "2S", "8S", "4C", "8C", "10C")
    ),
    (
      sequence: ("7D", "8D", "9D"),
      rest: ("8H", "5D", "2S", "8S", "4C", "8C", "10C", "JC", "QC")
    ),
    (
      sequence: ("10C", "JC", "QC"),
      rest: ("8H", "5D", "7D", "8D", "9D", "2S", "8S", "4C", "8C")
    ),
  )
)

#let smallhand(cards) = {
  for (card) in cards {
    deckz.mini(card)
  }
}

#let sequence_table(data) = {
  let rows = data.found.map(s => (
    [#smallhand(s.sequence)],
    [#smallhand(s.rest)]
  )).flatten()
  
  table(
    columns: (auto, auto),
    stroke: none,
    table.header([*sequence*], [*remaining hand*]),
    ..rows,
  )
}


= starting hand
#let hand = ("7D", "JC", "9D", "8H", "JH", "8C", "QC", "5D", "10C", "2S", "8S", "8D", "4C");
#smallhand(hand)
#table(
  columns: (auto, auto, auto, auto, auto),
  align: center + horizon,
  stroke: none,
  table.header(
    [current],
    [candidate],
    [same suit?], 
    [consecutive?],
    [valid?]
  ),
  [#deckz.mini("7D")], [#deckz.mini("JC")], [#emoji.crossmark], [#emoji.crossmark], [#emoji.crossmark],
  [#deckz.mini("7D")], [#deckz.mini("9D")], [#emoji.checkmark.box], [#emoji.crossmark], [#emoji.crossmark],
  [#deckz.mini("7D")], [#deckz.mini("8H")], [#emoji.crossmark], [#emoji.crossmark], [#emoji.crossmark],
)
#sequence_table(data)
