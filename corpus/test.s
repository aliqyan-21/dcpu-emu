set a 0x000a       ; load 10 into register a
set b 0x0005       ; load 5 into register b

; this is label
:loop
  sub a 1            ; subtract 1 from a
  add b 2            ; add 2 to b
  ife a 0            ; check if a has reached 0
  set pc end         ; if yes, exit loop

set pc loop        ; otherwise, jump back to loop

; this is another label
:end
  set [0x2000] b     ; store final result of b into memory address 0x2000
  set pc end ; infinite loop
