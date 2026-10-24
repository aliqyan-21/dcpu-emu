set a 0x1234
ife 0x1234 a     ; true, so the next line runs
set b 1
ifn 0x1234 a     ; false, so the next line is skipped
set c 1
:end
set pc end
