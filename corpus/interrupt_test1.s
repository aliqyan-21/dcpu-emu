; expect: a=1 b=0x42 c=5 sp=0 ia=0 pc=10 cycles=19
set a 1
ias handler
int 0x42
set c 5          ; runs after the handler returns
ias 0            ; remove the handler so the self-jump halts
:end
set pc end

:handler
set b a          ; A holds the message
rfi a
