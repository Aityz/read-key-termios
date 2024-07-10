//! Read Key (Termios): A tiny crate to read keys on Linux without introducing a huge amount of
//! dependencies.
//!
//! fn read_key(fd: i32) -> u8; Takes in the FD to read. Blocks until a key has been pressed.
//! This should be used with polling acceptably. It retuns a byte for the user to parse.
//!
//! fn init(); Enters raw mode, and allows the keys to start being read.
//!
//! fn close(): Enters cooked mode. Read_key() will not work after this.

/// The reason we don't use Bindgen to generate the structures is to keep this crate as small as
/// possible, to suit it's one purpose. Most users will know what this struct is, if they are
/// familiar with Libc. If, not I suggest you have a look.
#[repr(C)]
pub struct Termios {
    pub c_iflag: i32,
    pub c_oflag: i32,
    pub c_cflag: i32,
    pub c_lflag: i32,
}

static ECHO: i32 = 0o0000010;
static ICANON: i32 = 0x00002;
static TCSAFLUSH: i32 = 2;

extern "C" {
    fn tcgetattr(fd: i32, termios: &mut Termios);
    fn tcsetattr(fd: i32, int: i32, termios: &mut Termios);
    fn read(fd: i32, char: &mut u8, amount: i32);
}

/// Enters raw mode in the terminal so that the user can start to read keys properly. This uses
/// bindings to Termios to convert the terminal to raw mode.
///
/// # Safety
/// As it uses bindings to C, this will inherently be unsafe.
pub fn init() {
    let fd = std::os::fd::AsRawFd::as_raw_fd(&std::io::stdout()); // most likely 0

    unsafe {
        let mut termios: Termios = std::mem::zeroed();

        tcgetattr(fd, &mut termios);

        termios.c_lflag &= !(ECHO | ICANON); // disables ECHO and cooked mode

        tcsetattr(fd, TCSAFLUSH, &mut termios);
    }
}

/// Leaves raw mode in your terminal so that the user is returned to a normal terminal instance
/// (cooked mode).
///
/// # Safety
/// This calls functions in C, so it is unsafe.
pub fn close() {
    let fd = std::os::fd::AsRawFd::as_raw_fd(&std::io::stdout());

    unsafe {
        let mut termios: Termios = std::mem::zeroed();

        tcgetattr(fd, &mut termios);

        termios.c_lflag |= ECHO | ICANON; // re-enables echo

        tcsetattr(fd, TCSAFLUSH, &mut termios);
    }
}

/// Reads a key from the specified FD and returns it.
///
/// # Safety
/// This uses the function `read` from C.
pub fn read_key(fd: i32) -> u8 {
    let mut c: u8 = 0;
    unsafe {
        read(fd, &mut c, 1);
    }
    c
}
