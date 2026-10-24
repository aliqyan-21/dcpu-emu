; expect: a=0x2000 x=0x0040 i=0 pc=0x0020 cycles=128

; Notch's DCPU-16 reference test script

; Try some basic stuff
    set a 0x30
    set [0x1000] 0x20
    sub a [0x1000]
    ifn a 0x10
        set pc crash

; Do a loopy thing
    set i 10
    set a 0x2000
:loop
    set [0x2000+i] [a]
    sub i 1
    ifn i 0
        set pc loop

; Call a subroutine
    set x 0x4
    jsr testsub
    set pc crash

:testsub
    shl x 4
    set pc pop

; Hang forever. X should now be 0x40 if everything went right.
:crash
    set pc crash
