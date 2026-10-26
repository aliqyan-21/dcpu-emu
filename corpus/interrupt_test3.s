; expect: a=9 x=2 y=0x12 sp=0 ia=0
set a 9
ias handler
iaq 1
int 1
int 2
iaq 0
ias 0
:end
set pc end

:handler
add x 1
shl y 4          ; y = (y << 4) | message records the order
bor y a
rfi a
