# fills the display buffer one pixel at a time

x00
lbl $MAIN_LOOP_Y
x00
lbl $MAIN_LOOP_X
ldo x00 ldo x02 ldo x01 x01 and ldo x01 x01 and xor nez
jms $SET_PIXEL
drp drp drp
inc dup x20 ieq skp x03 $MAIN_LOOP_X sti
drp
inc dup x20 ieq skp x03 $MAIN_LOOP_Y sti
drp

x00 hlt

lbl $SET_PIXEL # set_pixel(x, y, bool); x < 0x20, y < 0x20, color = [0x00, 0xFF]
ldo x03 x07 and # get the offset of the bit in the byte of the display buffer (low 3 bits of x coordinate)
ldo x04 shr x03 # load x coordinate and shift right by 3
ldo x04 shl x02 # load y coordinate and shift left by 2 (the most significant bit will be a 0)
oor # bitwise both coordinates together to get a pointer to the display buffer
dup ldb # load the byte from the display buffer
x01 ldo x03 shl x00 # shift the bit to the left by the lower 3 bits of the x coordinate
ldo x05 skp x05 not and xFF skp x01 oor # set or clear the bit in the byte of the display buffer based on `bool`
swp stb # store the byte back to the display buffer
drp # drop the bit offset
rts # return from subroutine
