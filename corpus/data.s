; expect: a=1 pc=10 cycles=5

:start
 set a 0x0001
 set pc end

:message
  .dat "Hello"

:end
  set pc pc
