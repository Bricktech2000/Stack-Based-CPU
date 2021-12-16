# sends Hello World to stdout

# set index to 0
x00

lbl $LOOP
# load character from program memory
dup $STRING inc add ldp
# send current character to stdout and increment index
x00 out inc
# loop back if index is not equal to the length of the string
dup $STRING ldp ieq skp x03 $LOOP sti

hlt

# store string literal in program memory
lbl $STRING
p0D p48 p65 p6c p6c p6f p20 p57 p6f p72 p6c p64 p21 p0A
