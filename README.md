# gamingchair

This is an external *read-only* radar for Counter Strike 2 that utilizes [memflow](https://github.com/memflow/memflow) and [egui](https://github.com/emilk/egui).

# How can I use it?

First, you need to download a tool for managing plugins for memflow called [memflowup](https://github.com/memflow/memflowup).

You will need to download the plugin called `memflow-win32` and source your own connector to actually perform the read and writes undetected.

There are several public connectors for DMA cards and vulnerable drivers.

You will pass the connector that you want to use as an argument to the executable.

For example, `cargo run --release -- --connector winio`.

### Features & Preview

The map can be moved around and zoomed in/out of and the loadout bar is scrollable and adjustable.

Additionally, there is a bar that appears when the bomb is planted along with being defused and their respective fuses are displayed.

It will also draw an exclamation mark above a player holding a sniper rifle and a straight line from their view if they are scoped in.

Lastly, it will draw an image of the utility that a player is holding above their head.

<img width="1261" height="935" alt="image" src="https://github.com/user-attachments/assets/4a4291f6-5e42-4595-856d-07a4d7be0772" />

