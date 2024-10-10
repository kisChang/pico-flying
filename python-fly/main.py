from machine import Pin, PWM
from time import sleep
import network
import socket

class Servo:
    def __init__(self, pin_number, min_pulse=700, max_pulse=2450, frequency=50):
        self.servo_pin = PWM(Pin(pin_number, Pin.OUT))  # Set up PWM on the pin
        self.servo_pin.freq(frequency)         # Set frequency, default is 50Hz
        self.min_pulse = min_pulse
        self.max_pulse = max_pulse
        self.cycle = 1 / frequency * 1000000     # PWM cycle in us

    def angle(self, angle):
        pulse_width = self.min_pulse + (self.max_pulse - self.min_pulse) * angle / 180
        self.servo_pin.duty_u16(int(pulse_width * 65536 / self.cycle))  # Convert to duty for Pico's 16-bit resolution

    def deinit(self):
        self.servo_pin.deinit()

    def __del__(self):
        self.deinit()

servo_ym = Servo(11)
servo_lr = Servo(15)
servo_ud = Servo(16)


ap = network.WLAN(network.AP_IF)
ap.config(ssid='PICO_FLY', password='00001111')
ap.config(pm = 0xa11140)
ap.active(True)
while not ap.active():
    sleep(1)

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server_address = ('0.0.0.0', 10000)
print ('Server starting up on %s port %s' % server_address)
sock.bind(server_address)
sock.listen(1)
while True:
    print ('Server waiting for a connection...')
    connection, client_address = sock.accept()
    try:
        print ('Server connection from', client_address)
        buffer = ""
        while True:
            data = connection.recv(100)
            if data:
                buffer += data.decode('ascii')
                while ';' in buffer:
                    line, buffer = buffer.split(';', 1)
                    parts = line.split(',')
                    if len(parts) == 3:
                        dat = [int(parts[i]) for i in range(3)]
                        servo_ym.angle(dat[0])
                        servo_lr.angle(dat[1])
                        servo_ud.angle(dat[2])
            else:
                # 客户端主动断开连接时，data会为空
                print('Client disconnected.')
                break
    finally:
        # 断连后油门为0
        servo_ym.angle(0)
        connection.close()
