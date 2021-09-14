# Board Representation

All chess programs need an internal representation of the current state of the
game.

We'll code up an 0x88 board representation, which is similar to a mailbox system
common in other didactic chess engines.

|   |      |      |      |      |      |      |      |      |
| - | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- |
| 8 | 0x70 |      |      |      |      |      |      | 0x77 |
| 7 | 0x60 |      |      |      |      |      | 0x66 |      |
| 6 | 0x50 |      |      |      |      | 0x55 |      |      |
| 5 | 0x40 |      |      |      | 0x44 |      |      |      |
| 4 | 0x30 |      |      | 0x33 |      |      |      |      |
| 3 | 0x20 |      | 0x22 |      |      |      |      |      |
| 2 | 0x10 | 0x11 |      |      |      |      |      |      |
| 1 | 0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 |
|   | A    | B    | C    | D    | E    | F    | G    | H    |

Missing cells are left as an exercise to the reader :) (Kidding, TODO)
