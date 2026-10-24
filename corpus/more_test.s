set a 7
mdi a 3          ; A = 1
set b 0x8001
shl b 1          ; B = 0x0002, EX = 1
set [0x1000] 5
mli [0x1000] 2   ; [0x1000] = 10
add [0x1000] 1   ; [0x1000] = 11, must read exactly one address word
set c [0x1000]   ; C = 0x000b
:end
set pc end
