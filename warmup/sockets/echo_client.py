from socket import socket, AF_INET, SOCK_STREAM

def main(addr):
    sock = socket(AF_INET, SOCK_STREAM)
    sock.connect(addr)
    msg = input("Say >")
    sock.sendall(msg.encode('utf-8'))
    response = sock.recv(len(msg))
    print("Received >", response.decode('utf-8'))

main(('localhost', 12345))