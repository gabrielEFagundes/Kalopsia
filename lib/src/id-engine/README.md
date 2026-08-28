# id-engine

Engine used to create and manipulate identificators for the objects inside Kalopsia.

### Why?

Created because no-dependency rule of the challenge (chrono doesn't count, I'll remove it and implement my own datetime syscall soon).

Without a unique identificator, I'd have no way to deserialize data and map it correctly for retrieval.

Also, for serialization, the connection between 2 nodes would be absolutely gibberish, with an infinite recursive call because data allocating itself everytime.