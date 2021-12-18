# sends "Hello World!" to stdout

$STRING # push argument to stack
jms $PRINT_STRING # call subroutine
drp # drop argument from stack

x00 hlt # return 0 and halt


lbl $PRINT_STRING
x00 # set index to 0
lbl $PRINT_STRING_LOOP # for loop
dup ldo x03 inc add ldp # load character from program memory
x00 sta inc # send current character to stdout and increment index
dup ldo x03 ldp ieq skp x03 $PRINT_STRING_LOOP sti # loop back if index is not equal to the length of the string
drp # otherwise, drop the index and return from subroutine
rts


lbl $STRING # store string literal in program memory
p0D p48 p65 p6C p6C p6F p20 p57 p6F p72 p6C p64 p21 p0A
