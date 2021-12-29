# bit rotation proof-of-concept

x71 # load x71 on the stack
lbl $MAIN_LOOP
x00 # allocate the carry bit on the stack
swp # swap the carry bit and the value to rotate
slc x01 add # rotate the value left
jms $PRINT_BYTE_AS_BIN # print the value to stdout as hex
x0A xFF sta # print newline to stdout
$MAIN_LOOP sti


lbl $PRINT_BYTE_AS_BIN
ldo x02 # get the argument on the stack
x00 # allocate the counter on the stack
lbl $PRINT_BYTE_AS_BIN_LOOP
x00 ldo x02 slc x01 sto x02 # shift left, store back, and keep carry on the stack
$HEX_DIGITS ldo x02 adc ldp xFF sta # print the carry bit
drp # drop the carry bit
for x08 $PRINT_BYTE_AS_BIN_LOOP sti # loop back to next bit
drp # drop the counter
drp # drop the argument
rts # return from subroutine
lbl $HEX_DIGITS
p30 p31 p32 p33 p34 p35 p36 p37 p38 p39 p41 p42 p43 p44 p45 p46
