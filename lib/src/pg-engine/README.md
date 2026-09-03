# pg-engine

Engine is used to generate the position where the nodes will stay before being drawn into ratatui's `Canvas`.

**Fun Fact**: `pg` means *Position Generator*

### Why?

The nodes and edges need a coordinate of `x` and `y` to be represented on the `Canvas` widget from ratatui.

They could get random values when being created, this engine provides that too, but since I like things organized, I decided to write this engine to calculate different types of positionment, such as circular, random as said before, etc.