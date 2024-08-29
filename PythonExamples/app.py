# Example Solution for Fall 2024 CS Club Coding Competition Problems Month 1
# ICPC Mid-Atlantic USA 2022 Division 2
# Problem A:
# 
# Blueberry Waffle

# ================Step 0================
# Input and Variables

print("enter input:")
input_string = input()  

# split at the space between the numbers
r, f = input_string.split(" ")

# ================Step 1================
# Calculating Rotation

# python handles decimals automatically :)
flips = int(f) / int(r)

# ================Step 2================
# Horizontal Snap

# round to nearest integer
flips = round(flips)

# ================Step 3================
# Upfacing Side & Output

if(flips % 2 == 0):
    print("up")
else:
    print("down")

# # # Notes/Thoughts: # # #
# 
# Remember, this is *my* interpretation/solution, you may come at this in a different way
# with a different programming style, thats totally okay! This is also *very* verbose,
# to emphasize the problem-solving. Feel free to hyper-optimize your code.

# # # Reading and understanding the problem:
# 
# So, we're making a waffle in one of those spinny waffle maker machines.
# We want to check if, after some amount of time/rotation, the blueberries
# are facing up or down. The waffle starts with blueberries facing up, and
# then rotates 180 degrees (facing straight down) every r seconds, and stops
# rotating after f seconds. If it stops while not perfectly horizontal, it'll
# just snap to the nearest horizontal. 
# 
# In order to figure out which side is up, we need to calculate how much rotation
# occurs, figure out the closest horizontal orientation, and then output whether
# the blueberries are facing up or down.
# Fortunately, it is given that inputs will *not* allow for perfectly
# vertical stopping positions, so we can safely ignore that edge case.

# # # Input Structure:
# 
# One line:
# 
# r f
# 
# r: seconds for 180 degrees of rotation
# f: total time before stopping rotation
# 
# Fortunately, it is given that inputs will *not* allow for perfectly
# vertical stopping positions, so we can safely ignore that edge case.

# # # Output Structure:
#  
# One line:
# 
# if(blueberry side facing up) -> "up"
# else -> "down"

# # # Solving the problem:
# 
# We need to:
# 0) Gather input and store values, 1) calculate how much rotation occurs
# in the given time window, 2) figure out which way the machine will snap
# to once time ends, 3) calculate which side must be facing up and output
# our answer!
# 
# Step 0:  Input and Variables
#          In Python, we use the built-in input() function to take input.
#          We need two variables, r and f, so we take the input line
#          and parse it to find these two.
# 
# Step 1:  Calculating Rotation
#          We know that every r seconds, the machine rotates 180 degrees.
#          So by dividing f by r, we find out how many times 180 degrees
#          passes in the full timeframe. I store this in a 'flips' variable.
# 
# Step 2:  Horizontal Snap
#          We know that the machine snaps to the nearest horizontal,
#          meaning that if it rotated less than halfway, it'll snap back,
#          but if it rotated more than halfway, it'll snap forward,
#          meaning we can just *round the flips value to the nearest integer*!
#          This will give us exactly how many *full 180 degree flips* have
#          occurred in the given time!
# 
# Step 3:  Upfacing Side & Output
#          We know that the waffle starts with the blueberries facing up.
#          Meaning after 1 flip it'll face down, after 2 it'll be back up,
#          and so on. Therefor, the blueberries are facing up if 'flips'
#          is even, or down if it's odd!
# 
# # # # # # # # # # # # # #