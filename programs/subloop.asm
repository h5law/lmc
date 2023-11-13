# Given an input number subtract 1 from it until it becomes 0
# printing each number as you go along and exit after printing 999
        IN
loop    BRZ     exit
        OUT
        SUB     one
        BR      loop
exit    LDA     nines
        OUT
        HLT
one     DAT     001
nines   DAT     999
