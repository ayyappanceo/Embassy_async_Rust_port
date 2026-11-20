using Antmicro.Renode.Core;
using Antmicro.Renode.Peripherals.Bus;

namespace Antmicro.Renode.Peripherals.Miscellaneous
{
    public class Thejas32Spi : IDoubleWordPeripheral, IKnownSize
    {
        private uint rxByte;
        public Thejas32Spi(IMachine machine) { }
        public long Size => 0x100;

        public uint ReadDoubleWord(long offset) => offset == 0x04 ? 0x03u : (offset == 0x0C ? rxByte : 0u);
        public void WriteDoubleWord(long offset, uint value) { if (offset == 0x08) rxByte = (value & 0xFF) ^ 0xFF; }
        public void Reset() { rxByte = 0; }
    }

    public class Thejas32I2c : IDoubleWordPeripheral, IKnownSize
    {
        private uint sar, dr, regPtr;
        public Thejas32I2c(IMachine machine) { }
        public long Size => 0x100;

        public uint ReadDoubleWord(long offset) => offset == 0x04 ? 0x04u : (offset == 0x0C ? dr : 0u);
        public void WriteDoubleWord(long offset, uint value)
        {
            if (offset == 0x08) sar = value & 0x7F;
            else if (offset == 0x0C) regPtr = value & 0xFF;
            else if (offset == 0x00 && (value & 0x04) != 0 && sar == 0x68 && regPtr == 0x75) dr = 0x68;
        }
        public void Reset() { sar = dr = regPtr = 0; }
    }

    public class Thejas32Gpio : IDoubleWordPeripheral, IWordPeripheral, IKnownSize
    {
        private ushort data, direction;
        public Thejas32Gpio(IMachine machine) { }
        public long Size => 0x41000;

        public ushort ReadWord(long offset) => offset == 0x3FFFC ? data : (offset == 0x40000 ? direction : (ushort)0);
        public void WriteWord(long offset, ushort value)
        {
            if (offset == 0x3FFFC) data = value;
            else if (offset == 0x40000) direction = value;
        }

        public uint ReadDoubleWord(long offset) => ReadWord(offset);
        public void WriteDoubleWord(long offset, uint value) => WriteWord(offset, (ushort)(value & 0xFFFF));
        public void Reset() { data = direction = 0; }
    }
}
