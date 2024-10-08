
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
