# send Hello World to stdout

# load string literal to stack followed by length
x0A x21 x64 x6C x72 x6F x57 x20 x6F x6C x6C x65 x48 x0D

# send current character to stdout
swp x00 out dec
# loop back if length is not equal to zero
dup nez x0b and ldi sub sti

hlt
