:start
    set i message       ; Load address of the message into register I
    set a [i]           ; Read first character ('H' = 0x0048) into A
    add i 1             ; Increment pointer to next character
    set b [i]           ; Read second character ('e' = 0x0065) into B
    set pc pc           ; Halt / loop

:message
    .dat "Hello" 0x00
