read -p "Enter your serial port (if left empty will default to /dev/ttyUSB0): " tty_port

if [ -z "$tty_port" ]; then
  tty_port='/dev/ttyUSB0'
fi

echo "You selected $tty_port"
echo "Now flashing..."

sudo avrdude -pm328p -carduino -P$tty_port -b57600 -Uflash:w:$1