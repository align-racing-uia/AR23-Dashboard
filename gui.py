from concurrent.futures import thread
from distutils.util import change_root
from socket import SocketType
from tracemalloc import start
from turtle import width
import pyglet
import math
from can_io import *
from colors import *
from time import time,perf_counter
import threading
        

class DigitalGauge():
    def __init__(self, x, y, radius, min, max, layers, batch, backgroundColor = BACKGROUND, gaugeColor = HIGHVIS, gaugeColor2 = None, dataType="km/t", reverse = False, extended=False):
        self.max = max
        if extended:
            self.max_angle = math.pi*3/2
        else:
            self.max_angle = math.pi

        self.gaugeColor = gaugeColor
        self.gaugeColor2 = gaugeColor2
        self.dataType = dataType
        self.reversed = reverse
        self.pointer_holder = pyglet.shapes.Sector(x,y,radius, angle=-self.max_angle, start_angle=-(2*math.pi-self.max_angle), color=CONTAINER, group=layers[0], batch=batch)
        self.pointer = pyglet.shapes.Sector(x,y,radius, angle=-math.pi, start_angle=-(2*math.pi-self.max_angle), color=gaugeColor, group=layers[1], batch=batch)
        self.middle = pyglet.shapes.Sector(x,y,radius*8/10, angle=-self.max_angle, start_angle=-(2*math.pi-self.max_angle), color=backgroundColor, group=layers[2], batch=batch)
        self.text = pyglet.text.Label("0"+self.dataType, font_size=round(radius/5), anchor_y="center",anchor_x="center", x=x, y=y, color=(*gaugeColor,255), group=layers[3], batch=batch)

        if extended:
            self.min_text = pyglet.text.Label(str(min), font_size=round(radius/8), anchor_x="left",anchor_y="bottom", x=x+radius*5/100, y=y-radius, color=(*gaugeColor,255), group=layers[0], batch=batch)
            self.max_text = pyglet.text.Label(str(max), font_size=round(radius/8), anchor_x="right", anchor_y="top", x=x+radius, y=y, color=(*gaugeColor,255), group=layers[0], batch=batch)
            self.line = [
                pyglet.shapes.Line(x+radius*8/10,y, x+radius, y, 2, HIGHVIS, group=layers[2], batch=batch),
                pyglet.shapes.Line(x,y-radius*8/10, x, y-radius, 2, HIGHVIS, group=layers[2], batch=batch)
            ]
        else:
            self.min_text = pyglet.text.Label(str(min), font_size=round(radius/8), anchor_x="left",anchor_y="top", x=x-radius, y=y, color=(*gaugeColor,255), group=layers[0], batch=batch)
            self.max_text = pyglet.text.Label(str(max), font_size=round(radius/8), anchor_x="right", anchor_y="top", x=x+radius, y=y, color=(*gaugeColor,255), group=layers[0], batch=batch)
            self.line = [
                pyglet.shapes.Line(x+radius*8/10,y, x+radius, y, 2, HIGHVIS, group=layers[2], batch=batch),
                pyglet.shapes.Line(x-radius,y, x-radius*8/10, y, 2, HIGHVIS, group=layers[2], batch=batch)
            ]
    
    def update_pointer(self, new_data):
        percentage = new_data/self.max
        if self.gaugeColor2 != None:
            if percentage>=3/4 and not self.reversed:
                self.pointer.color = self.gaugeColor2
            elif percentage<=1/4:
                self.pointer.color = self.gaugeColor2
            else:
                self.pointer.color = self.gaugeColor

        self.pointer.angle = -self.max_angle*percentage
        self.text.text = str(new_data)+self.dataType


class AnalogGauge():
    def __init__(self, x, y, radius, min, max, layers, batch, info_text="", backgroundColor = BACKGROUND, gaugeColor = CONTAINER, gaugeColor2 = DANGER, dataType="km/t", reverse = False, extended=False):
        self.max = max
        self.x = x
        self.y = y
        self.radius = radius
        self.gaugeColor = gaugeColor
        self.gaugeColor2 = gaugeColor2
        self.dataType = dataType
        self.reversed = reverse
        self.pointer_holder = pyglet.shapes.Sector(x,y,radius, angle=math.pi, color=gaugeColor, group=layers[0], batch=batch)
        if not reverse:
            self.hot = pyglet.shapes.Sector(x,y,radius, angle=math.pi/4, color=gaugeColor2, group=layers[1], batch=batch)
        else:
            self.hot = self.hot = pyglet.shapes.Sector(x,y,radius, angle=math.pi/4, start_angle=math.pi*3/4, color=gaugeColor2, group=layers[1], batch=batch)
        self.middle = pyglet.shapes.Sector(x,y,radius*8/10, angle=math.pi, color=backgroundColor, group=layers[2], batch=batch)
        angle = 45
        self.pointer = pyglet.shapes.Triangle(x, y-radius/30, 
                                                x, y+radius/30, 
                                                x-+radius*0.8, y,
                                                color=HIGHVIS, group=layers[3], batch=batch)
        self.tmin = pyglet.text.Label(str(min)+self.dataType, font_size=round(radius/8), anchor_y="top",anchor_x="left", x=x-radius, y=y, color=(*HIGHVIS,255), group=layers[2], batch=batch)
        self.tmax = pyglet.text.Label(str(max)+self.dataType, font_size=round(radius/8), anchor_y="top",anchor_x="right", x=x+radius, y=y, color=(*HIGHVIS,255), group=layers[2], batch=batch)
        self.tinfo = pyglet.text.Label(info_text, font_size=round(radius/5), anchor_x="center", anchor_y="top", x=x, y=y-radius*1/10, color=(*HIGHVIS,255), group=layers[2], batch=batch)

    def update_pointer(self, new_data):
        angle = round(new_data/self.max*180)
        x = self.x
        y = self.y
        radius = self.radius
        self.pointer.x = x-radius/30*math.cos(math.radians(angle+90))
        self.pointer.x2 = x-radius/30*math.cos(math.radians(angle-90))
        self.pointer.x3 = x-radius*0.8*math.cos(math.radians(angle))
        self.pointer.y = y+radius/30*math.sin(math.radians(angle+90))
        self.pointer.y2 = y+radius/30*math.sin(math.radians(angle-90))
        self.pointer.y3 = y+radius*0.8*math.sin(math.radians(angle))

class Banner(pyglet.shapes.Rectangle):
    def __init__(self,x,y,width,height,layers,batch):
        super().__init__(x,y, width, height,color=OK,batch=batch, group=layers[0])
        self.anchor_y = self.height
        self.status = pyglet.text.Label("Normal operation", x=self.width/2, font_size=24, bold=True, y=y-self.height/2, color=(*TEXT, 255), anchor_x="center",anchor_y="center",batch=batch, group=layers[1])

class Lamp(pyglet.shapes.Rectangle):
    def __init__(self,text,x,y,width,height,layers,batch):
        super().__init__(x,y, width, height,color=BACKGROUND,batch=batch, group=layers[0])
        self.anchor_y = self.height
        self.lamp = pyglet.shapes.Rectangle(x+width*5/100,y, width*95/100, height,color=OK,batch=batch, group=layers[1])
        self.lamp.anchor_y = self.height
        self.text = pyglet.text.Label(text, x=x+width*5/100+self.lamp.width/2, y=y-(self.lamp.height/2), font_size=20, bold=True, anchor_x="center", anchor_y="center", color=(*TEXT, 255), group=layers[2], batch=batch)
        


class AR23GUI(pyglet.window.Window):
    def __init__(self):
        super(AR23GUI, self).__init__(800,480)

        pyglet.gl.glClearColor(75/255,75/255,75/255, 1) # Background color
        pyglet.gl.glLineWidth(8) # Line width for drawing

        self.queue = {
            "speed": [False,0],
            "soc": [False,0],
            "inverter_temp": [False,0],
            "motor_temp": [False,0],
            "battery_temp": [False,0]
        }

        self.layers = [pyglet.graphics.OrderedGroup(0), pyglet.graphics.OrderedGroup(1), pyglet.graphics.OrderedGroup(2),pyglet.graphics.OrderedGroup(3)]
        self.default_batch = pyglet.graphics.Batch()
        self.batches = [pyglet.graphics.Batch()]
        self.current_window = 0
        self.init_banner()
        self.init_lamps()
        self.init_endurance_screen()
        pyglet.clock.schedule_interval(self.force_draw,1/30)


    def init_banner(self):
        self.banner = Banner(0, self.height, self.width/2, self.height/6, self.layers, self.default_batch)

    def init_lamps(self):
        self.lamps = [
            Lamp("SFTY",self.width*3/6, self.height, self.width/6, self.height/6, self.layers, self.default_batch),
            Lamp("BSPD",self.width*4/6, self.height, self.width/6, self.height/6, self.layers, self.default_batch),
            Lamp("R2D",self.width*5/6, self.height, self.width/6, self.height/6, self.layers, self.default_batch),
        ]
        self.lamp_status = [
            True, True, False
        ]

    def init_endurance_screen(self):
        screen_batch = self.batches[0]
        self.speed_gauge = DigitalGauge(self.width/2, self.height*5/12, self.width/6, 0, 120, self.layers, screen_batch, extended=True)
        self.inverter_temp = AnalogGauge(self.width/6,self.height*5/6/8*5, self.width/8, 0, 60, self.layers, screen_batch, dataType="C", info_text="Inverter")
        self.motor_temp = AnalogGauge(self.width/6,self.height*5/6/8*1, self.width/8, 0, 60, self.layers, screen_batch, dataType="C", info_text="Motor")
        self.battery_temp = AnalogGauge(self.width*5/6,self.height*5/6/8*5, self.width/8, 0, 60, self.layers, screen_batch, dataType="C", info_text="Battery")
        self.soc = AnalogGauge(self.width*5/6,self.height*5/6/8*1, self.width/8, 0, 100, self.layers, screen_batch, reverse=True, dataType="%", info_text="Charge")
    
    def update_endurance_screen(self):
        if self.current_window != 0: return
        if self.queue["speed"][1]:
            self.speed_gauge.update_pointer(self.queue["speed"][1])
            self.queue["speed"][0] = False
        if self.queue["inverter_temp"][0]:
            self.inverter_temp.update_pointer(self.queue["inverter_temp"][1])
            self.queue["inverter_temp"][0] = False
        if self.queue["motor_temp"][0]:
            self.motor_temp.update_pointer(self.queue["motor_temp"][1])
            self.queue["motor_temp"][0] = False
        if self.queue["soc"][0]:
            self.soc.update_pointer(self.queue["soc"][1])
            self.queue["soc"][0] = False
        if self.queue["battery_temp"][0]:
            self.battery_temp.update_pointer(self.queue["battery_temp"][1])
            self.queue["battery_temp"][0] = False

   
    
    def on_draw(self):
        pyglet.clock.tick()
        self.clear()
        self.update_endurance_screen()
        self.default_batch.draw()
        self.batches[self.current_window].draw()
        print(pyglet.clock.get_fps())
    
    def force_draw(self,dt):
        pass

                

window = AR23GUI()
can_bus = CAN_io()
io = threading.Thread(target=can_bus.bus, args=(window.queue,))
io.start()
pyglet.app.run()
can_bus.stop()