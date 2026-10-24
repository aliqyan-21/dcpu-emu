; expect: a=1 b=2 c=0 x=7 y=8 z=0 ex=0 pc=0x001b cycles=28

set a 0
set b 2
set c 0
ife a 1          ; false (a=0), so skip the whole chain
ife b 2          ; skipped (it's an IFx, so skipping continues)
set c 3          ; skipped (ends the chain)
set x 7          ; runs

set a 1
ife a 1          ; true, falls through
ife b 9          ; false (b=2), skips the next instruction
set [0x1000] 5   ; skipped, and its 2 extra words must be stepped over
set y 8          ; runs
set z [0x1000]   ; z should still be 0 because the store never happened
:end
set pc end
