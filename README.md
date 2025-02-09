# UuidMap

This is an ArrayHashMap (otherwise called "Dense Storage" or an enhanced "VecMap").
It is essentially a relational database table.
This one is specialized to use a random u128 as the key, rather than a sequential index.

The performances are on-par with modern bitset-based entity-component-systems, while lagging behind archetypal ECS (~3x slower than [legion](https://github.com/amethyst/legion)).

However, it has the advantage that it does not constraint someone to follow the ECS pattern of entities and components, with anything else thrown into resources.
Rather, you can use this as a realtime relational database, suitable for game development and game engines.
This means you can store assets, entities, components, resources and events all in the same type `Table<T>`. Having it this way cleans up the code, allows for predictable performance and, most importantly, allows for unified tooling. (In-game `Table<ItemDefinition>` editor, anyone?)

You can also use it anywhere you would use an in-memory database.

### Where it makes sense to use it.

Where you have:
1. High performance needs
2. Need centralized data storage to avoid copies and/or pointers.
3. Data that will be shared between many systems.
4. Don't mind paying the (small) price of u128 indices in exchange for the consistency gain.

### Thread Safety
The tables are not thread-safe, by design.
You should be using another crate to orchestrate safe table access (no double mutable access, no reads during writes.)

For game engines, I recommend [world_dispatcher](https://github.com/AnneKitsune/world_dispatcher), which I made for this purpose.

### Complexities

- Iterating: O(n). raw `Vec<T>`.
- Inserting: O(n) amortized. 2x vec insert + 1x hashmap insert.
- Deleting: O(1). two swap remove. one hashmap get.
- Get element by key: O(1). hash of u128 + array access.
