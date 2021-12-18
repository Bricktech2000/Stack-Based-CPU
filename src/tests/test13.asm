# fills the display buffer one pixel at a time

x00
lbl $MAIN_LOOP_Y
x00
lbl $MAIN_LOOP_X
ldo x00 ldo x02 xFF
jms $SET_PIXEL
drp drp drp
inc dup x20 ieq skp x03 $MAIN_LOOP_X sti
drp
inc dup x20 ieq skp x03 $MAIN_LOOP_Y sti
drp

x00 hlt

lbl $SET_PIXEL
ldo x02 x00 sta x20 x00 sta ldo x03 x00 sta
rts
