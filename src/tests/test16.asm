# snake game

x07 x09 # allocate head position
x01 # allocate snake length
x00 x00 # allocate snake direction
# xFF x00 # allocate snake direction
x00 x00
lbl $MAIN_LOOP_STEP # main game loop

$SCORE jms $PRINT_STRING drp drp
ldo x01 jms $PRINT_BYTE_AS_HEX ldo x01 jms $PRINT_BYTE_AS_HEX drp drp

# dup inc x0F and nez skp x06 xcc dbg ldo x04 inc sto x04

x00 x00 # allocate position delta

# calculate position delta
drp ldo x03 not
dup add inc
ldo x05
skp x01 swp

# xFE lda
# nez not skp x01 hlt

# calculate the new head position by adding the position delta
ldo x08 ldo x02 add x0F and sto x08
ldo x07 ldo x01 add x0F and sto x07

drp drp # deallocate position delta

# copy snake direction to the new head position
ldo x03 x00 ldo x08 dup add inc ldo x08 jms $SET_ENCODED drp drp drp drp
ldo x02 x00 ldo x08 dup add nop ldo x08 jms $SET_ENCODED drp drp drp drp

# clear display buffer
# x00
# lbl $MAIN_LOOP_CLEAR
# dup x00 swp stb
# for x80 $MAIN_LOOP_CLEAR sti
# drp

ldo x06 ldo x06 # allocate temporary head position

ldo x06 neg # get negative snake length
lbl $MAIN_LOOP_BODY
x00 x00 # allocate position delta

# get direction bits
nop x00 ldo x05 dup add nop ldo x05 jms $GET_ENCODED drp drp drp # get first bit (up&down / left&right)
dup add inc
x00 x00 ldo x06 dup add inc ldo x06 jms $GET_ENCODED drp drp drp # get second bit (up / down or left / right)
skp x01 swp
# drp drp x00 x00 # HACK

xFF xFF ldo x06 dup add ldo x06 dup add jms $SET_ENCODED # display a pixel at the head position
# drp ldo x04 add ldo x06 dup add ldo x04 add jms $SET_ENCODED # display a pixel at the head position with the delta
drp drp drp drp

# calculate the new temporary head position by adding the position delta
ldo x04 ldo x02 add x0F and sto x04
ldo x03 ldo x01 add x0F and sto x03

drp drp # deallocate position delta
for x01 $MAIN_LOOP_BODY sti # loop while the snake length is not zero
drp

x00 xFF ldo x03 dup add ldo x03 dup add jms $SET_ENCODED # clear a pixel at the final head position (tail position)
drp drp drp drp
drp drp # deallocate temporary head position

x01 adc $MAIN_LOOP_STEP sti





# gets the encoded value of the cell at x, y through bit manipulation
lbl $GET_ENCODED # bool = get_encoded(is_display, x, y); x < 0x10, y < 0x10, bool = [0x00, 0xFF], is_display = [0x00, 0xFF]
jms $GET_MAGIC_VALUES
swp shr x00 x01 and # get the right bit based on the lower 3 bits of the x coordinate
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
lbl $HEX_DIGITS
p30 p31 p32 p33 p34 p35 p36 p37 p38 p39 p41 p42 p43 p44 p45 p46

lbl $SCORE
p08 p0D p53 p63 p6f p72 p65 p3A p20
