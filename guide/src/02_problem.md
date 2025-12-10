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
   .  . [@] @      Neighbors (5 valid, 3 out-of-bounds):
   @  @  @  .      ┌─────────────┐
   @  @  @  @      │ ·  ·  ·     │  · = out of bounds
                   │ .  ░  @     │  ░ = center (not counted)
                   │ @  @  @     │
                   └─────────────┘
```
  Neighbor @ count: 4 → ≥ 4, not accessible


Step 4: (0, 3) → @ ✓
```
   .  .  @ [@]     Neighbors (5 valid):
   @  @  @  .      ┌─────────────┐
   @  @  @  @      │ ·  ·  ·     │
                   │ @  ░  .     │
                   │ @  @  .     │
                   └─────────────┘
```                   
  Neighbor @ count: 3 → < 4, accessible! ✓

and so on...
