import socket

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
        while True:
            data = connection.recv()
            str = data.decode('ascii')
            print ('Server received "%s"' % str)
    finally:
        #断连后油门为0
        connection.close()
