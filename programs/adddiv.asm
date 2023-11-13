# Input two numbers, let say a and b.
# Print the quotient of the Euclidean division a/b.
# The robust version.

# **********************************

# ***********************
# input and store a and b
# ***********************

# Input and store the values of a and b.
        IN
        STO     a
        IN
        STO     b

# *************************
# initialise variable count
# *************************

# Set the value of count to zero.
# This is the only difference with the non-robust version
# (and that we have to define zero in the data section).

        LDA     zero
        STO     count


# *********
# main loop
# *********

# Successively subtract b's from a.
# We loop for as long as the value left in a is not negative.
# For every b we subtract from a we increment the variable count,
# that is, we add 1 to the quotient.

start   LDA     count
        ADD     one
        STO     count
        LDA     a
        SUB     b
        STO     a
        BRP     start

# **********************************
# final computation, print, and exit
# **********************************

# As we first increment count and then check if we finished,
# at the end of the above loop the count is the quotient +1.
# We subtract 1 to find the correct quotient, print, and halt.

done    LDA     count
        SUB     one
        OUT
        HLT

# ****
# data
# ****
a       DAT     000
b       DAT     000
count   DAT     000
one     DAT     001
zero    DAT     000
