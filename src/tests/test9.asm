# fibonacci sequence

# push argument on stack
x0A

# push 1 and 0 to stack
x01 x00

# jump out of loop if argument == 0
ldo x02 nez not x0B and ldi add sti
# run one fibonacci iteration
ldo x01 add swp
# decrement argument
ldo x02 dec sto x02
# loop back
x13 ldi sub sti

# return the result
sto x01 drp
hlt
