# Read Key (Termios)
This is a tiny crate that adds functionality for entering and leaving raw mode on Linux. It is useful when you are making a small CLI application and you don't want to add a ton of dependencies.
# Functions
``init()``: Enters raw mode by disabling ICANON and ECHO.<br>
``close()``: Exits raw mode by enabling ICANON and ECHO.<br>
``read_key(fd: i32) -> u8``: Reads a key from the FD specified.
