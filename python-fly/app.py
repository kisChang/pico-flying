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

servo_ym.angle(45)
sleep(1)
servo_ym.angle(90)
sleep(1)
servo_ym.angle(135)
sleep(1)
servo_ym.angle(180)
sleep(10)
servo_ym.angle(0)
sleep(1)