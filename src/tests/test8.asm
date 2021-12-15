fibonacci sequence

generate argument
ldv010C

push one and zero to stack
ldvF1 ldvF0

jump out of loop if argument is zero
ldo82 nez29 not30 ldvFB and32 ldi15-add20-sti16
run one fibonacci iteration
ldo81 add20 swp1B
decrement argument
ldo82 dec25 sto42
loop back to start of loop
ldv0113 ldi15-sub22-sti16

return the fibonacci number
sto41 drp1A
hlt02
