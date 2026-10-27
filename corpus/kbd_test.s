; devices: keyboard
; keys: 10:+0x90 20:+0x61 30:-0x61 40:-0x90 50:+0x62
; expect: a=1 c=0 x=0x41 y=0x62 z=0 i=0
set z 0x55
set i 50
:burn
sub i 1
ifn i 0
set pc burn
set a 1
hwi 0
set x c          ; 'a' typed while Shift held, so 'A' (0x41)
hwi 0
set y c          ; 'b' typed after Shift released, so 'b' (0x62)
hwi 0
set z c          ; buffer empty, so 0
:end
set pc end
