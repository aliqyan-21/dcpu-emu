; expect: a=7 b=1 pc=6 cycles=11
set a 7
int 0x42         ; IA is 0, so this does nothing
set b 1
:end
set pc end
