# conway's game of life simulation

# current generation is stored in bits from 0x00 to 0x80 in the display buffer
#    next generation is stored in bits from 0x00 to 0x80 in RAM
# each cell is a 1x1 squaree in the display buffer

# draw a glider
# xFF xFF
# x04 x03 jms $SET_ENCODED drp drp
# x05 x04 jms $SET_ENCODED drp drp
# x05 x05 jms $SET_ENCODED drp drp
# x04 x05 jms $SET_ENCODED drp drp
# x03 x05 jms $SET_ENCODED drp drp
# drp drp

# draw an R_pentomino
# xFF xFF
# x0F x0E jms $SET_ENCODED drp drp
# x10 x0E jms $SET_ENCODED drp drp
# x0E x0F jms $SET_ENCODED drp drp
# x0F x0F jms $SET_ENCODED drp drp
# x0F x10 jms $SET_ENCODED drp drp
# drp drp

# draw a Diehard and a glider
# the Diehard below has already advanced 2 generations
xFF xFF
x0C x08 jms $SET_ENCODED drp drp
x0C x09 jms $SET_ENCODED drp drp
x0D x09 jms $SET_ENCODED drp drp
x11 x09 jms $SET_ENCODED drp drp
x12 x09 jms $SET_ENCODED drp drp
x13 x09 jms $SET_ENCODED drp drp
x12 x08 jms $SET_ENCODED drp drp

x01 x0C jms $SET_ENCODED drp drp
x02 x0D jms $SET_ENCODED drp drp
x02 x0E jms $SET_ENCODED drp drp
x01 x0E jms $SET_ENCODED drp drp
x00 x0E jms $SET_ENCODED drp drp
drp drp


x00 x00
lbl $MAIN_LOOP_STEP # main game loop

$GENERATION jms $PRINT_STRING drp drp
x00 x00 ldo x03 ldo x03 jms $PRINT_4_BYTES_AS_DEC drp drp drp drp

x00
lbl $MAIN_LOOP_Y # loop over y coordinate
x00
lbl $MAIN_LOOP_X # loop over x coordinate

x00 # allocate room for neighbor count

# count neighbours
xFF
lbl $MAIN_LOOP_DY
xFF
lbl $MAIN_LOOP_DX
dup ldo x02 oor nez skp x01 inc # if dy = dx = 0, skip the iteration by incrementing the counter
ldo x02 # get the neighbor count
x00 xFF ldo x03 ldo x07 add x1F and ldo x05 ldo x09 add x1F and jms $GET_ENCODED drp drp drp # get_encoded(x + dx & 0x1F, y + dy & 0x1F)
neg add sto x02 # store the new neighbor count
for x02 $MAIN_LOOP_DX sti
drp
for x02 $MAIN_LOOP_DY sti
drp

# game of life rules
dup x03 ieq xFF ldo x03 ldo x05 # if 3 neighbours, set the cell to alive. otherwise, set it to dead
ldo x04 x02 ieq not skp x07 jms $GET_ENCODED x00 sto x02 jms $SET_ENCODED # exception: if the cell has 2 neighbours, do not take action
drp drp drp drp

drp # deallocate room for neighbor count
for x20 $MAIN_LOOP_X sti
drp
for x20 $MAIN_LOOP_Y sti
drp

# copy all cells from the next generation to the current generation
x00
lbl $MAIN_LOOP_N
dup dup lda swp stb
for x80 $MAIN_LOOP_N sti
drp

x01 adc $MAIN_LOOP_STEP sti






# gets the encoded value of the cell at x, y through bit manipulation
lbl $GET_ENCODED # bool = get_encoded(is_display, x, y); x < 0x10, y < 0x10, bool = [0x00, 0xFF], is_display = [0x00, 0xFF]
jms $GET_MAGIC_VALUES
shr x00 x01 and # get the right bit based on the lower 3 bits of the x coordinate
ldo x09 sub sto x08 # return `bool` from the subroutine (by subtracting it from the space allocated for the return value)
drp # drop the extra pointer to the buffer
drp drp # drop the return address of GET_MAGIC_VALUES
rts # return from subroutine

# sets the encoded value of the cell at x, y through bit manipulation
lbl $SET_ENCODED # draw_encoded(bool, is_display, x, y); x < 0x20, y < 0x20, bool = [0x00, 0xFF], is_display = [0x00, 0xFF]
jms $GET_MAGIC_VALUES
x01 swp shl x00 # shift the bit to the left by the lower 3 bits of the x coordinate
ldo x0A skp x05 not and xFF skp x01 oor # set or clear the bit in the byte of the buffer based on `bool`
swp ldo x08 xFF ieq skp x06 ldo x08 add sta xFF skp x01 stb # store the byte back to the right buffer based on is_display
drp # drop the return address of GET_MAGIC_VALUES
drp # drop the return address of GET_MAGIC_VALUES
rts # return from subroutine

# a helper function for the two subroutines above
lbl $GET_MAGIC_VALUES
ldo x05 shr x03 # load x coordinate and shift right by 3
ldo x05 shl x02 # load y coordinate and shift left by 2 (the most significant bit will be a 0)
oor # bitwise both coordinates together to get a pointer to the buffer
dup ldo x08 xFF ieq skp x06 ldo x08 add lda xFF skp x01 ldb # load the byte from the right buffer based on is_display
ldo x07 x07 and # get the offset of the bit in the byte of the buffer (low 3 bits of x coordinate)
ldo x04 ldo x04 rts

# sends a string to stdout
lbl $PRINT_STRING # print_string(length, [char])
x00 # set index to 0
lbl $PRINT_STRING_LOOP # for loop
ldo x04 ldo x04 ldo x02 adc x01 adc ldp # load character from program memory
xFF sta # send current character to stdout
inc ldo x04 ldo x04 ldp ldo x01 ieq skp x04 $PRINT_STRING_LOOP sti # loop back if index is not equal to the length of the string
drp # otherwise, drop the index and return from subroutine
rts

lbl $PRINT_BYTE_AS_HEX
$HEX_DIGITS ldo x04 shr x04 adc ldp xFF sta
$HEX_DIGITS ldo x04 x0F and adc ldp xFF sta
rts


# prints the 4-byte argument as decimal. the base used must be less than one byte large (0xFF, or 255).
lbl $PRINT_4_BYTES_AS_DEC # void print_4_bytes_as_dec(bytes[4])

x10 x10 x10 x10 x10 # allocate 5-character buffer on the stack
x00 x00 x00 x00 # allocate mod10
ldo x0E ldo x0E ldo x0E ldo x0E # allocate div10

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

# add the least significant byte of mod10 (shifted right by 1) to the character buffer
ldo x0B sto x0C
ldo x0A sto x0B
ldo x09 sto x0A
ldo x08 sto x09
ldo x04 shr x01 sto x08

# $HEX_DIGITS ldo x06 shr x01 adc ldp xFF sta
# clear the least significant byte of mod10
x00 sto x04
# while div10 is not zero, loop back to the next division
ldo x03 ldo x03 ldo x03 ldo x03 oor oor oor nez not skp x04 $PRINT_4_BYTES_AS_DEC_LOOP sti
# otherwise,
drp drp drp drp # drop div10
drp drp drp drp # drop mod10
# print the 5-character buffer to stdout
$HEX_DIGITS ldo x02 adc ldp xFF sta drp
$HEX_DIGITS ldo x02 adc ldp xFF sta drp
$HEX_DIGITS ldo x02 adc ldp xFF sta drp
$HEX_DIGITS ldo x02 adc ldp xFF sta drp
$HEX_DIGITS ldo x02 adc ldp xFF sta drp
rts # return from subroutine


lbl $HEX_DIGITS
p30 p31 p32 p33 p34 p35 p36 p37 p38 p39 p41 p42 p43 p44 p45 p46 p20

lbl $GENERATION
p0C p0D p47 p65 p6E p65 p72 p61 p74 p69 p6f p6E p20
