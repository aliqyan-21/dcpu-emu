; expect: a=9 b=0 c=1 x=1 y=0x10 sp=0 ia=0 pc=14 cycles=28
set a 9
ias handler
iaq 1            ; queue interrupts
int 0x10         ; must NOT run the handler yet
set b x          ; x is still 0, so b = 0
iaq 0            ; queued interrupt fires right after this
set c x          ; handler already ran, so c = 1
ias 0
:end
set pc end

:handler
add x 1
set y a
rfi a
