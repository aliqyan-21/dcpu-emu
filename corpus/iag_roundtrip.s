; expect: a=0x1234 b=0x4321 ia=0 pc=8 cycles=10
ias 0x1234       ; set IA
iag a            ; A = IA
ias 0x4321       ; change IA
iag b            ; B must be the new value
ias 0            ; clear IA so the self-jump below counts as a halt
:end
set pc end
