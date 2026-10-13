```
starting state: next = [item=[card] rest=[rest of the cards]]

example:
--------
next = [
   item=[2H] rest=[1H 3H 4H]
]
completed = []

while let Some((sequence, rest)) = next.pop():

loop(item=[2H] rest=[1H 3H 4H]):
  continuations (2H):
    2H.sequence(1H) -> []
    2H.sequence(3H) -> item=[2H, 3H] rest=[1H, 4H]
    2H.sequence(4H) -> []
  next = [
    item=[2H, 3H] rest=[1H, 4H]
  ]
  completed = []

 loop(item=[2H, 3H] rest=[1H, 4H]):
   continuations (3H):
     3H.sequence(1H) -> []
     3H.sequence(4H) -> [3H, 4H] -> item=[2H, 3H, 4H] rest=[1H]
   next = [
     item=[2H, 3H, 4H] rest=[1H]
   ]
   completed = [
     item=[2H, 3H, 4H] rest=[1H]
   ]

loop(item=[2H, 3H, 4H] rest=[1H]):
  ending continuations:
    4H.sequence(1H) -> []
  next = []
  completed = [
     item=[2H, 3H, 4H] rest=[1H]
  ]

example:
--------
next = [
   item=[2H] rest=[1H 3H 4H]
]
completed = []

while let Some((sequence, rest)) = next.pop():

loop(item=[2H] rest=[1H 3H 4H]):
  continuations (2H):
    2H.sequence(1H) -> []
    2H.sequence(3H) -> item=[2H, 3H] rest=[1H, 4H]
    2H.sequence(4H) -> []
  next = [
    item=[2H, 3H] rest=[1H, 4H]
  ]
  completed = []

 loop(item=[2H, 3H] rest=[1H, 4H]):
   continuations (3H):
     3H.sequence(1H) -> []
     3H.sequence(4H) -> [3H, 4H] -> item=[2H, 3H, 4H] rest=[1H]
   next = [
     item=[2H, 3H, 4H] rest=[1H]
   ]
   completed = [
     item=[2H, 3H, 4H] rest=[1H]
   ]

loop(item=[2H, 3H, 4H] rest=[1H]):
  ending continuations:
    4H.sequence(1H) -> []
  next = []
  completed = [
     item=[2H, 3H, 4H] rest=[1H]
  ]
```
