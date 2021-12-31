# binary-to-decimal converter

x00 x00 x06 xC1 jms $PRINT_4_BYTES_AS_DEC drp drp drp drp
# x00 x00 x00 x34 jms $PRINT_4_BYTES_AS_DEC drp drp drp drp
x00 hlt


# print the 4-byte argument as decimal. the base used must be less than one byte large (0xFF, or 255).
lbl $PRINT_4_BYTES_AS_DEC # void print_4_bytes_as_dec(bytes[4])

x00 x00 x00 x00 # allocate mod10
ldo x09 ldo x09 ldo x09 ldo x09 # allocate div10

lbl $PRINT_4_BYTES_AS_DEC_LOOP

x00 # allocate counter
lbl $PRINT_4_BYTES_AS_DEC_DIVISION

x01 # set the carry bit
# subtract with borrow 10 from the least significant byte of mod10
ldo x06 x0A sbc
# run the positive-negative check
ldo x01 nez skp x04 $PRINT_4_BYTES_AS_DEC_IGNORE_STORE sti
# if the result is positive, store it back into mod10 and set the carry bit to 0
sto x06
xFF skp x01 # ignore the drop instruction below
lbl $PRINT_4_BYTES_AS_DEC_IGNORE_STORE
# otherwise, drop it from the stack and set the carry bit to 1
drp
# use carry on the stack to rotate left, store back, and get new carry onto the stack
# the fact that all those instructions are required to rotate 8 bytes left is a bit concerning
x00 ldo x03 slc x01 ldo x02 add sto x03 swp drp
x00 ldo x04 slc x01 ldo x02 add sto x04 swp drp
x00 ldo x05 slc x01 ldo x02 add sto x05 swp drp
x00 ldo x06 slc x01 ldo x02 add sto x06 swp drp
x00 ldo x07 slc x01 ldo x02 add sto x07 swp drp
x00 ldo x08 slc x01 ldo x02 add sto x08 swp drp
x00 ldo x09 slc x01 ldo x02 add sto x09 swp drp
x00 ldo x0A slc x01 ldo x02 add sto x0A swp drp
drp # drop the last carry bit

for x21 $PRINT_4_BYTES_AS_DEC_DIVISION sti # loop back to next divisionn 32 times as there are 32 bits in 4 bytes
drp

# print the least significant byte of mod10 to stdout (shifted right by 1 bit)
$HEX_DIGITS ldo x06 shr x01 adc ldp xFF sta
# clear the least significant byte of mod10
x00 sto x04
# while div10 is not zero, loop back to the next division
ldo x03 ldo x03 ldo x03 ldo x03 oor oor oor nez not skp x04 $PRINT_4_BYTES_AS_DEC_LOOP sti
# otherwise,
drp drp drp drp # drop div10
drp drp drp drp # drop mod10
rts # return from subroutine

lbl $HEX_DIGITS
p30 p31 p32 p33 p34 p35 p36 p37 p38 p39 p41 p42 p43 p44 p45 p46