
/* kernel::serial */
/* UART communication model
 * Based on the Arduino Serial API */

pub type baud = u32;

pub type serialReceiveHandler<'a> = 'a|&u8, uint| -> ();

pub trait Serial{
    /// Initialize device and begin transmission.
    fn open(&mut self, baud) -> bool;
    fn isOpen(&self) -> bool;

    /// End transmission, close device.
    fn close(&mut self) -> bool;

    /// Number of bytes available to read
    fn available(&self) -> uint; 
    
    /// Read up to length bytes into buffer. Return number of bytes read.
    fn readBuf(&mut self, buffer : &mut u8, length : uint) -> uint;
    fn readChar(&mut self, c : &mut char) -> uint;

    /// Write a single character. Return number of bytes written.
    fn write(&self, char) -> uint;

    /// Write a buffer of bytes. Return number of bytes written.
    fn writeBuf(&self, buffer : &u8, length : uint) -> uint;

    fn flush(&self) -> uint;

    /// Callback on new data available.
    fn addReceiveHandler(&self, serialReceiveHandler) -> bool;

    /// Remove all receive handlers
    fn clearReceiveHandlers(&self) -> ();
}

