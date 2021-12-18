# asks for single character input and prints it back

$PROMPT # push argument to stack
ldi x05 add # push return address to stack
$PRINT_STRING sti # call function
drp # drop argument from stack

x00 # allocate memory for return value
ldi x05 add # push return address to stack
$GET_CHAR sti # call function

$ANSWER # push argument to stack
ldi x05 add # push return address to stack
$PRINT_STRING sti # call function
drp # drop argument from stack

x00 sta x0A x00 sta # send character followed by newline to stdout

x00 hlt # return 0 and halt


lbl $PRINT_STRING
x00 # set index to 0
lbl $PRINT_STRING_LOOP # for loop
dup ldo x03 inc add ldp # load character from program memory
x00 sta inc # send current character to stdout and increment index
dup ldo x03 ldp ieq skp x03 $PRINT_STRING_LOOP sti # loop back if index is not equal to the length of the string
drp # otherwise, drop the index and return from the function
sti

lbl $GET_CHAR
x00 lda sto x01 # get char input from stdin and store to return value
sti # return from the function


lbl $PROMPT # store string literal in program memory
p13 p45 p6E p74 p65 p72 p20 p61 p20 p63 p68 p61 p72 p61 p63 p74 p65 p72 p3A p20 # Enter a character: 
lbl $ANSWER # store string literal in program memory
p0D p59 p6F p75 p20 p65 p6E p74 p65 p72 p65 p64 p3A p20 # You entered: 
