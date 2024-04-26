# ===================
# === DESCRIPTION ===
# ===================

# This program takes three numbers as input and then
# outputs the arithmetic mean of the three numbers.
#
# In order to deal with potential overflows and the
# unexpected behaviour that may result from them,
# the program calculates the average of each number
# individually by dividing each number by three.
# And then sums the averages together to get the
# final mean.
#
# The program uses a carry variable to keep track
# of the remainder of the division by 3. This
# carry variable is then divided by three and this
# average is then added to the final mean to get
# the correct arithmetic mean.

# ===================
# === INITIALISER ===
# ===================

# Initialise the variables used in the program
# This is technically not needed as the variables
# are already initialised to zero, but it acts as
# a sanity check to ensure they are what we expect

        LDA    div    # load 3
        STO    count  # reset loop count
        SUB    div    # subtract 3
        STO    carry  # reset carry
        SUB    div    # subtract 3
        SUB    mul    # subtract 1
        STO    mean   # set mean to -4 to offset greedy division

# ===================
# = Arithmetic Mean =
# ===================

# First calculate the average of each number
# individually by performing division by 3 and
# incrementing the average by three each time.
# Doing each number separately allows us to
# avoid any potential overflow issues that
# may cause unexpected behaviour

next    LDA    count  # load the loop count
        BRZ    cloop  # if zero, goto carry loop
        SUB    mul    # subtract 1
        STO    count  # store the loop count
        IN            # input the next number
        STO    curr   # store the current value
start   LDA    mean   # load the current average
        ADD    mul    # add 1
        STO    mean   # store the current average
        LDA    curr   # load the current value
        SUB    div    # subtract 3
        STO    curr   # store the current value
        BRP    start  # if positive, repeat
        ADD    div    # add 3 to get remainder
        ADD    carry  # add the carry
        STO    carry  # store the carry
        BR     next   # go to next loop

# Finally we need to calculate the average of the
# carry variable. We do this by dividing the carry
# by three and adding the result to the final mean

# Maximum carry value is 2+2+2=6 which is ~2
# loops through the carry division

cloop   LDA    mean   # load the mean 
        ADD    mul    # add 1
        STO    mean   # store the mean 
        LDA    carry  # load the carry
        SUB    div    # subtract 3
        STO    carry  # store the carry
        BRP    cloop  # if positive, repeat

# ===================
# == Final Outputs ==
# ===================

exit    LDA    mean   # load the mean average
        OUT           # output the mean

# ===================
# ==== Variables ====
# ===================

count   DAT     000   # final value is 000 so acts as HLT
mul     DAT     001
div     DAT     003
curr    DAT     000
mean    DAT     000
carry   DAT     000
