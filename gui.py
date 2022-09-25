from cgitb import text
from ctypes import pointer
import pygame
import sys
import threading
import canreader
from pygameEvent import *
import math

class UI:
    # The function that initializes everything
    def __init__(self, title, width, height, can_io):
        pygame.init()
        self.can_io = can_io
        self.clock = pygame.time.Clock()
        print("Initializing UI")
        self.screen_size = self.width, self.height = width, height
        self.bg = BACKGROUND
        self.screen = pygame.display.set_mode(self.screen_size)
        pygame.display.set_caption(title)
        self.init_text()
        self.last_battery_state = 40
        self.lamps = {
            "R2D": False,
            "BSPD": True,
            "SFTY": True
        }
        self.current_throttle = 0
        self.current_brake = 0
        self.data = {
            "Motor_Temp": [0, "C"],
            "Inverter_Temp": [0, "C"],
            "Battery_Temp": [0, "C"],
            "Battery_Voltage": [0, "V"],
            "Battery_SOC": [0, "%"],
            "Inverter_LV": [0, "V"],
            "Motor_Speed": [0, "RPM"],
            "Wheel_Speed": [0,"km/t"],
            "Throttle": [0, "%"],
            "Brake_Pressure":[0, "BAR"]
        }
        self.log = ["Normal operation."]
        #self.tabs =  ["driver", "data", "logs"]
        self.current_window = -1
        #switches from -1 to 0, to enter driver screen
        self.switch_window()
        self.can_io.start()
        

    def switch_window(self):
        self.screen.fill(pygame.Color(self.bg))
        self.current_window += 1
        if self.current_window > 2:
            self.current_window = 0
        match self.current_window:
            case 1:
                self.update_banner()
                self.update_lamps()
                self.update_data_screen()
            case 2:
                self.update_banner()
                self.update_lamps()
                self.update_logging_screen()
                #self.current_window = -1
            case _:
                self.update_banner()
                self.update_lamps()
                self.init_brake_status()
                self.init_driver_data()
                self.init_throttle_status()
                self.update_throttle_status()
                


    # Initializes the different text sizes and fonts used
    def init_text(self):
        self.font = pygame.font.Font(None, 128)
        self.font_small = pygame.font.Font(None, 64)
        self.font_xsmall = pygame.font.Font(None, 48)
        self.font_xxsmall = pygame.font.Font(None, 32)
        self.font_xxxsmall = pygame.font.Font(None, 24)

    # Initializes the progress bars on each side of the screen
    def init_brake_status(self):
        self.brake_status_container_rect = pygame.Rect((0,0), (self.screen.get_width()/8, (self.screen.get_height()*5/6)*6/8))
        self.brake_status_container_rect.centery = self.screen.get_height()/2 + self.screen.get_height()/6/2
        self.brake_status_container_rect.centerx = (self.screen.get_width()-self.screen.get_width()*2/3)/4
        brake_status_container_glow = pygame.Rect((0,0), ((self.brake_status_container_rect.width)*104/100, (self.brake_status_container_rect.height)*102/100))
        brake_status_container_glow.centerx = self.brake_status_container_rect.centerx
        brake_status_container_glow.centery = self.brake_status_container_rect.centery

        #Text over brake
        brake_text = self.font_xsmall.render("BRK", True, HIGHVIS)
        brake_text_rect = brake_text.get_rect()
        brake_text_rect.y = brake_status_container_glow.y-brake_text_rect.height
        brake_text_rect.centerx = self.brake_status_container_rect.centerx
        self.screen.blit(brake_text, brake_text_rect)

        pygame.draw.rect(self.screen, HIGHVIS, brake_status_container_glow)
        pygame.draw.rect(self.screen, CONTAINER, self.brake_status_container_rect)
        self.update_brake_status()

    def update_brake_status(self):
        brake = round(self.data["Brake_Pressure"][0]/9*100)

        if brake-self.current_brake != 0:
            self.current_brake += (brake-self.current_brake)*(1-abs(brake-self.current_brake)/100)*1/100       

        percentage = self.current_brake
        # avoids some bugs
        if percentage < 0 or percentage > 100 or self.current_window>0:
            return

        # clears the old green from the battery container
        pygame.draw.rect(self.screen, CONTAINER, self.brake_status_container_rect)

        # draws the green box inside the battery container, and places it at the bottom
        self.brake_status_rect = pygame.Rect((0,0), (self.brake_status_container_rect.width*85/100,self.brake_status_container_rect.height*19/20*percentage/100))
        self.brake_status_rect.centery = self.brake_status_container_rect.centery + (self.brake_status_container_rect.height*19/20/2) * (100-percentage)/100
        self.brake_status_rect.centerx = self.brake_status_container_rect.centerx

        pygame.draw.rect(self.screen, POWER, self.brake_status_rect)
    
    def init_throttle_status(self):
        self.throttle_status_container_rect = pygame.Rect((0,0), (self.screen.get_width()/8, (self.screen.get_height()*5/6)*6/8))
        self.throttle_status_container_rect.centery = self.screen.get_height()/2 + self.screen.get_height()/6/2
        self.throttle_status_container_rect.centerx = self.screen.get_width() -(self.screen.get_width()-self.screen.get_width()*2/3)/4
        throttle_status_container_glow = pygame.Rect((0,0), ((self.throttle_status_container_rect.width)*104/100, (self.throttle_status_container_rect.height)*102/100))
        throttle_status_container_glow.centerx = self.throttle_status_container_rect.centerx
        throttle_status_container_glow.centery = self.throttle_status_container_rect.centery

        #Text over throttle
        throttle_text = self.font_xsmall.render("THR", True, HIGHVIS)
        throttle_text_rect = throttle_text.get_rect()
        throttle_text_rect.y = throttle_status_container_glow.y-throttle_text_rect.height
        throttle_text_rect.centerx = self.throttle_status_container_rect.centerx
        self.screen.blit(throttle_text, throttle_text_rect)

        pygame.draw.rect(self.screen, HIGHVIS, throttle_status_container_glow)
        pygame.draw.rect(self.screen, CONTAINER, self.throttle_status_container_rect)
        self.update_throttle_status()

    def update_throttle_status(self):

        if self.data["Throttle"][0]-self.current_throttle != 0:
            self.current_throttle += (self.data["Throttle"][0]-self.current_throttle)*(1-abs(self.data["Throttle"][0]-self.current_throttle)/100)*1/100

        percentage = self.current_throttle
        # avoids some bugs
        if percentage < 0 or percentage > 100 or self.current_window>0:
            return

        # clears the old green from the power container
        pygame.draw.rect(self.screen, CONTAINER, self.throttle_status_container_rect)

        # draws the green box inside the power container, and places it at the bottom
        self.throttle_status_rect = pygame.Rect((0,0), (self.throttle_status_container_rect.width*85/100,self.throttle_status_container_rect.height*19/20*percentage/100))
        self.throttle_status_rect.centery = self.throttle_status_container_rect.centery + (self.throttle_status_container_rect.height*19/20/2) * (100-percentage)/100
        self.throttle_status_rect.centerx = self.throttle_status_container_rect.centerx

        pygame.draw.rect(self.screen, OK, self.throttle_status_rect)

    def draw_gauge(self, center, text_input, data, max, min=0, invert=False):
        radius = self.screen.get_height()/6

        text_color = GAUGE_TEXT

        if data[0]/max > 75/100 and not invert:
            pygame.draw.circle(self.screen, DANGER, center, radius*104/100)
            pygame.draw.circle(self.screen, ALERT, center, radius)
            text_color = pygame.Color("black")
        elif data[0]/max < 25/100 and invert:

            pygame.draw.circle(self.screen, DANGER, center, radius*104/100)
            pygame.draw.circle(self.screen, ALERT, center, radius)
            text_color = pygame.Color("black")
        else:
            pygame.draw.circle(self.screen, HIGHVIS, center, radius*104/100)
            pygame.draw.circle(self.screen, CONTAINER, center, radius)

        
        unit = self.font_xxsmall.render(data[1], True, text_color)
        unit_rect = unit.get_rect()
        unit_rect.centerx = center[0]
        unit_rect.centery = center[1] + radius*4/5
        text = self.font_xxsmall.render(text_input, True, text_color)
        text_rect = text.get_rect()
        text_rect.centerx = center[0]
        text_rect.centery = center[1] + radius/2

        min_text = self.font_xxxsmall.render(str(min)+data[1], True, text_color)
        min_text_rect = min_text.get_rect()
        min_text_rect.x = center[0]-radius*0.9
        min_text_rect.centery = center[1]+min_text_rect.height

        max_text = self.font_xxxsmall.render(str(max)+data[1], True, text_color)
        max_text_rect = max_text.get_rect()
        max_text_rect.x = center[0]+radius*0.9 - max_text_rect.width
        max_text_rect.centery = center[1]+max_text_rect.height

        self.screen.blit(min_text,min_text_rect)
        self.screen.blit(max_text,max_text_rect)
        self.screen.blit(text, text_rect)
        self.screen.blit(unit, unit_rect)

        for i in range(0,13):
            angle = 180/12 * i
            factor = 0.8
            if angle % 45 == 0:
                factor = 0.7

            color = text_color
            if not invert and angle > 140:
                color = pygame.Color("red")
            if invert and angle < 40:
                color = pygame.Color("red")

            base = pointer_tip = (center[0]-(radius*factor*math.cos(math.radians(angle))), 
            center[1]-(radius*factor*math.sin(math.radians(angle))))
            tip = pointer_tip = (center[0]-(radius*0.9*math.cos(math.radians(angle))), 
            center[1]-(radius*0.9*math.sin(math.radians(angle))))
            pygame.draw.line(self.screen, color, base, tip, 2)

        angle = data[0]/max * 180
        pointer_tip = (center[0]-(radius*0.9*math.cos(math.radians(angle))), 
            center[1]-(radius*0.9*math.sin(math.radians(angle))))
        pygame.draw.line(self.screen, pygame.Color("Red"), center, pointer_tip, 3)
        

    # Shows all the temperatures on the driver page
    def init_driver_data(self):
        self.update_driver_data()

    # Updates the data on the driver page
    def update_driver_data(self):
        if(self.current_window>0): return
        #text_display = pygame.Rect((self.screen.get_width()/12,self.screen.get_height()*3/12), (self.screen.get_width()*7/11, self.screen.get_height()*4/6))
        text_display = pygame.Rect((0,0), (self.screen.get_width()*3/5, self.screen.get_height()*6/10))
        text_display.centery = self.brake_status_container_rect.centery
        text_display.centerx = self.screen.get_width()/2
        #pygame.draw.rect(self.screen, HIGHVIS, text_display )

        top_box = pygame.Rect((0,text_display.y), (text_display.width*199/200, (text_display.height/3)*96/100))
        #battery_box.centerx = text_display.centerx
        top_box.centery = text_display.y + text_display.height/6 + (text_display.height/3)*2/100
        top_box.centerx = text_display.centerx
        #pygame.draw.rect(self.screen, CONTAINER, battery_box)

        bottom_box = pygame.Rect((0,text_display.y), (text_display.width*199/200, (text_display.height/3)*96/100))
        #battery_box.centerx = text_display.centerx
        bottom_box.centery = text_display.y + text_display.height*5/6 - (text_display.height/3)*1/100
        bottom_box.centerx = text_display.centerx
        #pygame.draw.rect(self.screen, CONTAINER, motor_box)

        
        battery_text = self.font_xsmall.render("BATTERY: " + str(self.data["Battery_Temp"][0]) + "C", True, HIGHVIS)
        inverter_text = self.font_xsmall.render("INVERTER: " + str(self.data["Inverter_Temp"][0]) + "C", True, HIGHVIS)
        # motor_text = self.font_xsmall.render("MOTOR: " + str(self.data["Motor_Temp"][0]) + "C", True, HIGHVIS)


        # motor_rect = motor_text.get_rect()
        # motor_rect.centery = motor_box.centery
        # motor_rect.centerx = motor_box.centerx-text_display.width*1/400

        radius = self.screen.get_height()/6
        self.draw_gauge((top_box.centerx-bottom_box.width*3/8,top_box.centery), "Inverter",self.data["Inverter_Temp"], 70)
        self.draw_gauge((top_box.centerx+bottom_box.width*3/8,top_box.centery), "Motor",self.data["Motor_Temp"], 70)

        self.draw_gauge((top_box.centerx,top_box.centery), "Inverter",self.data["Inverter_Temp"], 70)
        self.draw_gauge((bottom_box.centerx,bottom_box.centery), "Motor",self.data["Motor_Temp"], 70)

        self.draw_gauge((bottom_box.centerx-bottom_box.width*3/8,bottom_box.centery), "Battery",self.data["Battery_Temp"], 70)
        self.draw_gauge((bottom_box.centerx+bottom_box.width*3/8,bottom_box.centery), "Battery",self.data["Battery_SOC"], 100, 0, True)


        # self.screen.blit(battery_text, battery_rect)
        # self.screen.blit(inverter_text, inverter_rect)
        # self.screen.blit(motor_text, motor_rect)
        

    # Initializes the lamps in the top right corner
    def update_lamps(self, lamp=None, value=False):
        #if self.current_window > 0: return
        if lamp != None:
            self.lamps[lamp] = value
        offset = 0
        for i in self.lamps:
            lamp_background_rect = pygame.Rect((0,0), (self.screen.get_width()*1/6, self.screen.get_height()*1/6))
            lamp_background_rect.x = self.screen.get_width() - lamp_background_rect.width - (offset*lamp_background_rect.width)
            pygame.draw.rect(self.screen, TEXT, lamp_background_rect)
            lamp_rect = pygame.Rect((0,0), (self.screen.get_width()*1/6*95/100, self.screen.get_height()*1/6))
            lamp_rect.x= lamp_background_rect.x+self.screen.get_width()*1/6*5/100
            lamp_rect.centery = lamp_background_rect.centery
            if self.lamps[i]:
                pygame.draw.rect(self.screen, OK, lamp_rect)
            else:
                pygame.draw.rect(self.screen, DANGER, lamp_rect)
            
            lamp_text = self.font_xsmall.render(i, True, TEXT)
            lamp_text_rec = lamp_text.get_rect()
            lamp_text_rec.centerx = lamp_rect.centerx
            lamp_text_rec.centery = lamp_rect.centery
            self.screen.blit(lamp_text, lamp_text_rec)

            offset += 1

    # Banner displays system status on top of the page
    #
    #   Status | 0 |   1   |    2    |
    #            ok warning  critical
    #
    #
    def update_banner(self,text = "SYSTEM OK",status = 0):
        banner_rect = pygame.Rect((0,0), (self.screen.get_width()/2, self.screen.get_height()/6))
        if status == 0:
            pygame.draw.rect(self.screen, OK, banner_rect)
        elif status == 1:
            pygame.draw.rect(self.screen, ALERT, banner_rect)
        else:
            pygame.draw.rect(self.screen, DANGER, banner_rect)
        
        banner_text = self.font_xsmall.render(text, True, TEXT)
        banner_text_rect = banner_text.get_rect()
        banner_text_rect.center = banner_rect.center
        self.screen.blit(banner_text, banner_text_rect)
 ####################### DATA SCREEN ########################
    def update_data_screen(self):
        if self.current_window != 1: return
        screen_width = self.screen.get_width()
        screen_height = self.screen.get_height()
        info_screen_rect = pygame.Rect((screen_width*0.025,screen_height*1/6+screen_height*5/6*0.025), (screen_width*0.95,screen_height*5/6*0.95))
        pygame.draw.rect(self.screen, self.bg, info_screen_rect)
        ################## HERE COMES THE DATA HANDLING ###############
        x = 0
        y = 0
        text_height = self.font_xxsmall.render("q", False, HIGHVIS).get_rect().height
        for data_point in self.data:
            new_string = data_point.replace("_", " ")
            data_text = self.font_xxsmall.render(new_string+": "+str(self.data[data_point][0])+self.data[data_point][1], True, HIGHVIS)
            self.screen.blit(data_text, (info_screen_rect.x+(x*screen_width/2), info_screen_rect.y+text_height*(y+1)))
            y += 1
            if y>12:
                y = 0
                x += 1
################ LOGGING SCREEN ######################
    def update_logging_screen(self):
        if self.current_window != 2: return
        screen_width = self.screen.get_width()
        screen_height = self.screen.get_height()
        log_screen_rect = pygame.Rect((screen_width*0.025,screen_height*1/6+screen_height*5/6*0.025), (screen_width*0.95,screen_height*5/6*0.95))
        pygame.draw.rect(self.screen, self.bg, log_screen_rect)
        y = 1
        log_text = self.font_xsmall.render("Log:", False, HIGHVIS)
        text_height = log_text.get_rect().height
        self.screen.blit(log_text,(log_screen_rect.x, log_screen_rect.y))
        for line in self.log:
            log_text = self.font_xsmall.render(line, True, HIGHVIS)
            self.screen.blit(log_text, (log_screen_rect.x, log_screen_rect.y+text_height*y))
            y += 1
            
        


    def main_loop(self):
        while True:
            self.clock.tick()
            #print(self.clock.get_fps())
            #print("tick")
            if(self.current_throttle != self.data["Throttle"][0]):
                self.update_throttle_status()
            if(self.current_throttle != round(self.data["Brake_Pressure"][0]/9*100)):
                self.update_brake_status()
            for event in pygame.event.get():
                #print("event: " + str(event.type))
                if event.type == UPDATE_APPS:
                    self.lamps["R2D"] = event.data[0]
                    self.data["Throttle"][0] = event.data[1]
                    self.data["Brake_Pressure"][0] = event.data[2]
                    self.update_throttle_status()
                    self.update_brake_status()
                    self.update_driver_data()
                    self.update_lamps()
                elif event.type == UPDATE_BATTERY: 
                    #print(event.data)
                    self.data["Battery_SOC"][0] = event.data[0]
                    self.data["Battery_Voltage"][0] = event.data[1]
                    self.update_driver_data()
                elif event.type == UPDATE_BATTERY_TEMP:
                    #print(event.data)
                    self.data["Battery_Temp"][0] = event.data
                    self.update_driver_data()
                    self.update_data_screen()
                elif event.type == UPDATE_MOTOR_TEMP:
                    self.data["Motor_Temp"][0] = event.data
                    self.update_driver_data()
                    self.update_data_screen()
                elif event.type == UPDATE_INVERTER_TEMP:
                    self.data["Inverter_Temp"][0] = event.data
                    self.update_driver_data()
                    self.update_data_screen()
                elif event.type == UPDATE_INVERTER_PCB_TEMP:
                    self.data["Inverter_Pcb_Temp"][0] = event.data
                    self.update_data_screen()
                elif event.type == UPDATE_CELL_VOLTAGE:
                    self.data["Cell_Voltage"][0] = event.data
                    self.update_data_screen()
                elif event.type == UPDATE_INVERTER_12V:
                    self.data["Inverter_LV"][0] = event.data
                    self.update_data_screen()
                elif event.type == UPDATE_MOTOR_SPEED:
                    self.data["Motor_Speed"][0] = event.data[0]
                    self.data["Wheel_Speed"][0] = event.data[1]
                    self.update_data_screen()
                elif event.type == pygame.QUIT: 
                    self.can_io.stop_process()
                    sys.exit()
                elif event.type == pygame.KEYDOWN:
                    if event.key == pygame.K_SPACE:
                        self.switch_window()
            pygame.display.flip()
            
