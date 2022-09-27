import can
import cantools
import pyglet
from time import time,perf_counter




class AR23GUI(pyglet.window.Window):
    def __init__(self):
        super(AR23GUI, self).__init__()
        self.i = 0
        self.framerate = pyglet.text.Label(text='Unknown', font_name='Verdana', font_size=8, x=10, y=10, color=(255,0,0,50))
        self.speed_label = pyglet.text.Label(str(self.i), font_size=48,bold=True, anchor_x="center", x=self.width/2, y=self.height/2, color=(255,0,0,255))

    def on_draw(self):
        t1 = perf_counter()*1000
        self.speed_label.text = str(self.i)
        self.clear()
        for i in range(0,5):
            self.speed_label.y = self.height/2 + 50 * i
            self.speed_label.draw()
        print(perf_counter()*1000-t1)
    
    def on_mouse_press(self, x, y, button, modifiers):
        self.i += 1
    
    def on_mouse_motion(self, x, y, dx, dy):
        pass



window = AR23GUI()
pyglet.app.run()
print("test")