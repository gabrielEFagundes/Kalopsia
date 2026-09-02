# ss-engine

Engine used to serialize structs and save them to a file.

Also used to do the opposite.

**Fun-fact**: ss means "struct serializer".

### Why?

I had to create this small parsing lib because this is a pretty darn complex graph engine, with quite a few nesting structs.

This means that, without an automated parsing library, the `write!` macro would get messy very, **VERY** quickly.