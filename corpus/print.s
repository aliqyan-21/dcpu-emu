:start
    set i message       ; load address of the message into register I
    set a [i]           ; read first character ('H' = 0x0048) into A
    add i 1             ; increment pointer to next character
    set b [i]           ; read second character ('e' = 0x0065) into B
    set pc pc           ; halt

:message
    .dat "Hello" 0x00
