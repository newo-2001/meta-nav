# Meta-Nav

*Meta-Nav* turns an esp32 microcontroller into a primitive "cat detector."
It does this by periodically measuring the pressure on a connected pressure plate.
If a cat is detected it will send notifications to multiple channels to inform the owner that the cat wants to be let inside.

# Building

Building the project requires the espressif rust fork and an LLVM fork that supports Xtensa targets.
Instructions on how to install these tools can be found [here](https://esp-rs.github.io/book/installation/riscv-and-xtensa.html).