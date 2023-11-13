# Get the minimum of two input values
        IN
        STO     a
        IN
        STO     b
        SUB     a
        BRP     exita
        LDA     b
        BR      exitb
exita   LDA     a
exitb   OUT
        HLT
a       DAT     000
b       DAT     000
