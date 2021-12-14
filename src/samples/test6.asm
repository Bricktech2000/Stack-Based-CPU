fibonacci sequence

generate argument
val010C

point work pointer to argument
ldp13 stp-s-w54
push one and zero to stack
val0101 val0100

jump out of loop if value at work pointer is zero
dup-w-s99 eqz29 val010A and32 inc24-ldi15-add20-sti16
run one fibonacci iteration
dug1B-drp1A add20 swp1D
decrement memory at work pointer
dec-w-wE5

loop back to start of loop
val0112 ldi15-sub22-sti16

otherwise, return fibonacci number
swp1D drp1A swp1D drp1A htl02
