import os
import math
import time

A, B, C = 0, 0, 0
cube_width = 20
width, height = 160, 44
z_buffer = [0] * (width * height)
buffer = [' '] * (width * height)
backgroud_ascii = ord(' ')
increment_speed = 1
x, y, z = 0, 0, 0

cam_distance = 100
ooz = 0
xp, yp = 0, 0
K1 = 40
idx = 0

def calcx(i, j, k):
    return j * math.sin(A) * math.sin(B) * math.cos(C) - k * math.cos(A) * math.sin(B) * math.cos(C) + j * math.cos(A) * math.sin(C) + k * math.sin(A) * math.sin(C) + i * math.cos(B) * math.cos(C)

def calcy(i, j, k):
    return j * math.cos(A) * math.cos(C) + k * math.sin(A) * math.cos(C) - j * math.sin(A) * math.sin(B) * math.sin(C) + k * math.cos(A) * math.sin(B) * math.sin(C) - i * math.cos(B) * math.sin(C)

def calcz(i, j, k):
    return k * math.cos(A) * math.cos(B) - j * math.sin(A) * math.cos(B) + i * math.sin(B)

def calc_surface(cubeX, cubeY, cubeZ, ch):
    global x, y, z, ooz, xp, yp, idx
    x = calcx(cubeX, cubeY, cubeZ)
    y = calcy(cubeX, cubeY, cubeZ)
    z = calcz(cubeX, cubeY, cubeZ) + cam_distance
    ooz = 1 / z
    xp = int(width / 2 - 2 * cube_width + K1 * x * ooz * 2)
    yp = int(height / 2 - K1 * y * ooz)
    idx = xp + yp * width
    if 0 <= idx < width * height:
        if ooz > z_buffer[idx]:
            z_buffer[idx] = ooz
            buffer[idx] = ch

def clear_screen():
    os.system('clear' if os.name == 'posix' else 'cls')

def print_cube():
    clear_screen()
    for k in range(width * height):
        print(buffer[k], end='\n' if k % width == width - 1 else '')

while True:
    buffer = [' '] * (width * height)
    z_buffer = [0] * (width * height)

    for cubeX in range(-cube_width, cube_width, int(increment_speed)):
        for cubeY in range(-cube_width, cube_width, int(increment_speed)):
            calc_surface(cubeX, cubeY, -cube_width, '.')
            calc_surface(cube_width, cubeY, cubeX, ';')
            calc_surface(-cube_width, cubeY, -cubeX, ':')
            calc_surface(-cubeX, cubeY, cube_width, ',')
            calc_surface(cubeX, -cube_width, -cubeY, '+')
            calc_surface(cubeX, cube_width, cubeY, '-')

    print_cube()
    A += 0.09
    B += 0.09
    time.sleep(0.1)

