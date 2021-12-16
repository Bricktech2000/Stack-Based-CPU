# sends Hello World to stdout

# set index to 0
x00
# load character from program memory
dup x17 add ldp
# send current character to stdout and increment index
x00 out inc
# loop back if index is not equal to the length of the string
dup x16 ldp ieq not x14 and ldi sub sti

hlt

# store string literal in program memory
p0D p48 p65 p6c p6c p6f p20 p57 p6f p72 p6c p64 p21 p0A
