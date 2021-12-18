# conway's game of life simulation

# x06 x14 sta
# x00 x00 x04 x05 jms $GET_ENCODED p03
# drp
# x01 hlt
# xFF x00
# x04 x03 jms $SET_ENCODED drp drp
# x05 x04 jms $SET_ENCODED drp drp
# x05 x05 jms $SET_ENCODED drp drp
# x04 x05 jms $SET_ENCODED drp drp
# x03 x05 jms $SET_ENCODED drp drp
# drp drp
xFF x04 sta

# x00
lbl $MAIN_LOOP_STEP

x05
lbl $MAIN_LOOP_Y
x04
lbl $MAIN_LOOP_X

x00 # allocate room for neighbor count

xFF
lbl $MAIN_LOOP_DY
xFF
lbl $MAIN_LOOP_DX
dup ldo x02 oor nez skp x01 inc
ldo x02
x00 x00 ldo x03 ldo x07 add ldo x05 ldo x09 add jms $GET_ENCODED p03
neg add sto x02
inc dup x02 ieq skp x03 $MAIN_LOOP_DX sti
drp
inc dup x02 ieq skp x03 $MAIN_LOOP_DY sti
drp

dup x03 ieq x20 ldo x03 ldo x05
ldo x04 x02 ieq skp x06 jms $SET_ENCODED
p04

ldo x01 ldo x03
jms $DRAW_CELL
p05

drp # deallocate room for neighbor count
inc dup x0F ieq skp x03 $MAIN_LOOP_X sti
drp
inc dup x0F ieq skp x03 $MAIN_LOOP_Y sti
drp

x00
lbl $MAIN_LOOP_N
dup x20 add lda ldo x01 sta
inc dup x20 ieq skp x03 $MAIN_LOOP_N sti
drp

# inc dup xFF ieq skp x03 $MAIN_LOOP_STEP sti
$MAIN_LOOP_STEP sti
# drp

x00 hlt


lbl $DRAW_CELL # draw_cell(x, y); x < 0x10, y < 0x10
# draws a 2x2 square at x, y in the display buffer
x00 x00 ldo x04 ldo x04 jms $GET_ENCODED drp dup add ldo x04 dup add xFF sto x02
jms $SET_ENCODED
inc jms $SET_ENCODED
swp inc swp jms $SET_ENCODED
dec jms $SET_ENCODED
p04
rts

lbl $GET_ENCODED # bool = get_encoded(is_display, x, y); x < 0x10, y < 0x10, bool = [0x00, 0xFF], is_display = [0x00, 0xFF]
jms $GET_MAGIC_VALUES
swp shr x00 x01 and # get the right bit based on the lower 3 bits of the x coordinate
nez sto x06 # return `bool` from the subroutine
drp # drop the extra pointer to the buffer
drp # drop the return address of GET_MAGIC_VALUES
rts # return from subroutine

lbl $SET_ENCODED # draw_encoded(bool, is_display, x, y); x < 0x20, y < 0x20, bool = [0x00, 0xFF], is_display = [0x00, 0xFF]
jms $GET_MAGIC_VALUES
x01 swp shl x00 # shift the bit to the left by the lower 3 bits of the x coordinate
ldo x08 skp x05 not and xFF skp x01 oor # set or clear the bit in the byte of the buffer based on `bool`
swp ldo x06 xFF ieq skp x06 ldo x06 add sta xFF skp x01 stb # store the byte back to the right buffer based on is_display
drp # drop the return address of GET_MAGIC_VALUES
rts # return from subroutine

lbl $GET_MAGIC_VALUES
ldo x03 shr x03 # load x coordinate and shift right by 3
ldo x03 shl x01 ldo x06 xFF ieq skp x02 shl x01 # load y coordinate and shift left by 1 or 2 (the most significant bit will be a 0)
oor # bitwise both coordinates together to get a pointer to the buffer
dup ldo x06 xFF ieq skp x06 ldo x06 add lda xFF skp x01 ldb # load the byte from the right buffer based on is_display
ldo x05 x07 and # get the offset of the bit in the byte of the buffer (low 3 bits of x coordinate)
ldo x03 rts
