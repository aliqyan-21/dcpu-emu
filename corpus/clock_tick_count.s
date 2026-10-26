; devices: clock
; expect: a=2 b=0x77 x=3 y=0x77 i=0 pc=24

ias handler      ; install the handler
set a 0
set b 1
hwi 0            ; clock at 60 Hz
set a 2
set b 0x77       ; interrupt message
hwi 0            ; enable interrupts
set i 700
:burn
sub i 1
ifn i 0
set pc burn
ias 0            ; remove the handler so the self-jump below counts as a halt
:end
set pc end

:handler
add x 1          ; count interrupts
set y a          ; A holds the message
rfi 0
