from concurrent.futures import thread
from distutils.util import change_root
from socket import SocketType
import can
import cantools
import pyglet
from time import time,perf_counter
import threading


filter = {
    int("0x00a5", 16): {"can_id": 0x00a5, "can_mask": 0xFFFFFFF, "extended": False, "db": "cascadia"}
}
    
db = {
    "cascadia": cantools.database.load_file("./cascadia.dbc")
}

kill = False

def can_io(storage):
    can_bus = can.Bus(interface="socketcan", channel="vcan0", can_filters=list(filter.values()))
    for msg in can_bus:
        if kill: break
        data = db[filter[msg.arbitration_id]["db"]].decode_message(msg.arbitration_id, msg.data)
        storage.append([msg.arbitration_id, data])
        


class AR23GUI(pyglet.window.Window):
    def __init__(self):
        super(AR23GUI, self).__init__()
        self.speed = 0
        self.queue = []
        self.framerate = pyglet.text.Label(text='Unknown', font_name='Verdana', font_size=8, x=10, y=10, color=(255,0,0,50))
        self.speed_label = pyglet.text.Label(str(self.speed), font_size=48,bold=True, anchor_x="center", x=self.width/2, y=self.height/2, color=(255,0,0,255))
        pyglet.clock.schedule_interval(self.handle_can_io, 1/1000)
        

    def on_draw(self):
        t1 = perf_counter()*1000
        self.speed_label.text = str(self.speed)
        self.clear()
        self.speed_label.draw()
        print(perf_counter()*1000-t1)
    
    def handle_can_io(self, dt):
        print(len(self.queue))
        while len(self.queue)>0:
            q = self.queue.pop()
            if q[0] == int("0x00a5", 16):
                self.speed = str(round(q[1]["D2_Motor_Speed"] * 0.0314256))+"km/t"
    
    def on_close(self):
        kill = True
        return super().on_close()




window = AR23GUI()
io = threading.Thread(target=can_io, args=(window.queue,))
io.start()
pyglet.app.run()
