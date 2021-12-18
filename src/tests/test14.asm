# conway's game of life simulation

# x06 x14 sta
# x00 x00 x04 x05 jms $GET_ENCODED drp drp drp
# drp
# x01 hlt

x00
lbl $MAIN_LOOP_Y
x00
lbl $MAIN_LOOP_X
ldo x00 ldo x02
jms $DRAW_CELL
drp drp
inc dup x10 ieq skp x03 $MAIN_LOOP_X sti
drp
inc dup x10 ieq skp x03 $MAIN_LOOP_Y sti
drp

x00 hlt

lbl $DRAW_CELL # draw_cell(x, y); x < 0x10, y < 0x10
# draws a 2x2 square at x, y in the display buffer
x00 x00 ldo x04 ldo x04 jms $GET_ENCODED drp dup add ldo x04 dup add xFF sto x02
jms $SET_ENCODED
inc jms $SET_ENCODED
swp inc swp jms $SET_ENCODED
dec jms $SET_ENCODED
drp drp drp drp
rts

lbl $GET_ENCODED # bool = get_encoded(is_display, x, y); x < 0x10, y < 0x10, bool = [0x00, 0xFF], is_display = [0x00, 0xFF]
ldo x02 shr x03 # load x coordinate and shift right by 3
ldo x02 shl x02 # load y coordinate and shift left by 2 (the most significant bit will be a 0)
oor # bitwise both coordinates together to get a pointer to the buffer
ldo x04 skp x04 lda xFF skp x01 ldb # load the byte from the right buffer based on is_display
ldo x03 x07 and # get the offset of the bit in the byte of the buffer (low 3 bits of x coordinate)
swp shr x00 x01 and # get the right bit based on the lower 3 bits of the x coordinate
nez sto x04 # return `bool` from the subroutine
rts # return from subroutine

lbl $SET_ENCODED # draw_encoded(bool, is_display, x, y, bool); x < 0x20, y < 0x20, bool = [0x00, 0xFF], is_display = [0x00, 0xFF]
ldo x02 shr x03 # load x coordinate and shift right by 3
ldo x02 shl x02 # load y coordinate and shift left by 2 (the most significant bit will be a 0)
oor # bitwise both coordinates together to get a pointer to the buffer
dup ldo x05 skp x04 lda xFF skp x01 ldb # load the byte from the right buffer based on is_display
ldo x04 x07 and # get the offset of the bit in the byte of the buffer (low 3 bits of x coordinate)
x01 swp shl x00 # shift the bit to the left by the lower 3 bits of the x coordinate
ldo x07 skp x05 not and xFF skp x01 oor # set or clear the bit in the byte of the buffer based on `bool`
swp ldo x05 skp x04 sta xFF skp x01 stb # store the byte back to the right buffer based on is_display
rts # return from subroutine
