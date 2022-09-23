pub trait Serial {
    fn read(&mut self);
    fn write(&mut self);
    fn set_baudrate(&mut self, baud: Baudrate) -> Result<Baudrate, Error>;
    fn set_parity(&mut self, parity: bool) -> Result<bool, Error>;
    fn set_data_bits(&mut self, data_bits: DataBits) -> Result<DataBits, Error>;
    fn set_stop_bits(&mut self, stop_bits: StopBits) -> Result<StopBits, Error>;
}

pub enum Baudrate {
    B50,
    B75,
    B110,
    B134,
    B150,
    B200,
    B300,
    B600,
    B1200,
    B1800,
    B2400,
    B4800,
    B9600,
    B19200,
    B28800,
    B38400,
    B57600,
    B76800,
    B115200,
    B230400,
    B460800,
    B576000,
    B921600,
}

pub enum DataBits {
    D5,
    D6,
    D7,
    D8,
    D9,
}

pub enum StopBits {
    S1,
    S2,
}

pub enum Error {
    Overrun,
    Underrun,
    Framing,
    Parity,
    BreakCondition,
}
