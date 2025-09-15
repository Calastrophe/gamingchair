# gamingchair

This is an external *read-only* radar for Counter Strike 2 that utilizes [memflow](https://github.com/memflow/memflow) and [egui](https://github.com/emilk/egui).

# How can I use it?

First, you need to download a tool for managing plugins for memflow called [memflowup](https://github.com/memflow/memflowup).

You will need to download the plugin called `memflow-win32` and source your own connector to actually perform the read and writes undetected.

There are several public connectors for DMA cards and vulnerable drivers.

You will pass the connector that you want to use as an argument to the executable.

For example, `cargo run --release -- --connector winio`.
