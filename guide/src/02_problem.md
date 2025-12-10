## Problem Summary

**Input:** A grid where @ = paper roll, . = empty

**Task:** Count paper rolls that have fewer than 4 neighboring paper rolls (8-directional adjacency)

## Iteration Walkthrough

Grid (with coordinates):
```raw
       0123456789
     ┌──────────
   0 │ ..@@.@@@@.
   1 │ @@@.@.@.@@
   2 │ @@@@@.@.@@
   ...
```


Step 1: (0, 0) → .
```
  [.] .  @  @     ← current cell marked with [ ]
   @  @  @  .
   @  @  @  @
```   
  Not a @, skip.


Step 2: (0, 1) → .
```
   . [.] @  @
   @  @  @  .
   @  @  @  @
```   
  Not a @, skip.


Step 3: (0, 2) → @ ✓
```
   .  . [@] @      Neighbors (5 in bounds, 3 out-of-bounds):
   @  @  @  .      ┌─────────────┐
   @  @  @  @      │ x  x  x     │  x = out of bounds
                   │ .  ░  @     │  ░ = center (not counted)
                   │ @  @  .     │
                   └─────────────┘
```
  Neighbor @ count: 3 → < 4, vaild! ✓


Step 4: (0, 3) → @ ✓
```
   .  .  @ [@]     Neighbors (3 in bounds):
   @  @  @  .      ┌─────────────┐
   @  @  @  @      │ x  x  x     │
                   │ @  ░  x     │
                   │ @  .  x     │
                   └─────────────┘
```                   
  Neighbor @ count: 2 → < 4, vaild! ✓

Step 6: (1, 1) → @ ✓
```
   .  .  @  @     Neighbors (8 in bounds):
   @ [@] @  .      ┌─────────────┐
   @  @  @  @      │ .  .  @     │
                   │ @  ░  @     │
                   │ @  @  @     │
                   └─────────────┘
```                   
  Neighbor @ count: 6 → < 4, invalid!

and so on...
