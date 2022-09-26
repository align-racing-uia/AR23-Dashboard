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
        self.current_gauges = {

        }
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
        self.current_window = 3
        #switches from -1 to 0, to enter driver screen
        self.switch_window()
        self.can_io.start()
        

    def switch_window(self):
        self.screen.fill(pygame.Color(self.bg))
        self.update_banner()
        self.update_lamps()
        pygame.event.set_blocked(pygame.MOUSEMOTION)
        self.current_window += 1
        if self.current_window > 4:
            self.current_window = 0

        if self.current_window == 1:
            self.update_data_screen()
        elif self.current_window == 2:
            pass
        elif self.current_window == 3:
            self.update_logging_screen()
        elif self.current_window == 4:
            self.update_endurance()
        else:
            self.init_driver_data()
                


    # Initializes the different text sizes and fonts used
    def init_text(self):
        self.font = pygame.font.Font(None, 128)
        self.font_small = pygame.font.Font(None, 64)
        self.font_xsmall = pygame.font.Font(None, 48)
        self.font_xxsmall = pygame.font.Font(None, 32)
        self.font_xxxsmall = pygame.font.Font(None, 24)

    def draw_gauge(self, radius, center, text_input, data_name, max, min=0, invert=False, degrees = 180):
        data = self.data[data_name]
        if not data_name in self.current_gauges:
            self.current_gauges[data_name] = 0

        self.current_gauges[data_name] = self.data[data_name][0]

        
        text_color = GAUGE_TEXT

        if data[0]/max > 75/100 and not invert:
            pygame.draw.circle(self.screen, pygame.Color(55,35,35), (center[0], center[1]+6), radius*108/100)
            pygame.draw.circle(self.screen, DANGER, center, radius*104/100)
            pygame.draw.circle(self.screen, ALERT, center, radius)
            text_color = pygame.Color("black")
        elif data[0]/max < 25/100 and invert:
            pygame.draw.circle(self.screen, pygame.Color(55,35,35), (center[0], center[1]+6), radius*108/100)
            pygame.draw.circle(self.screen, DANGER, center, radius*104/100)
            pygame.draw.circle(self.screen, ALERT, center, radius)
            text_color = pygame.Color("black")
        else:
            pygame.draw.circle(self.screen, SHADOW, (center[0], center[1]+6), radius*108/100)
            pygame.draw.circle(self.screen, HIGHVIS, center, radius*104/100)
            pygame.draw.circle(self.screen, CONTAINER, center, radius)

        font = self.font_xxsmall 
        font_height = font.render("test", True, pygame.Color("black")).get_rect().height
        if radius < font_height*4:
            font = self.font_xxxsmall
        elif radius > font_height*6:
            font = self.font_small
            text_color = HIGHVIS

        if degrees < 181:
            unit = font.render(data[1], True, text_color)
            unit_rect = unit.get_rect()
            unit_rect.centerx = center[0]
            unit_rect.centery = center[1] + radius*4/5
            text = font.render(text_input, True, text_color)
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
            
        else:
            text = font.render(text_input, True, text_color)
            text_rect = text.get_rect()
            text_rect.centerx = center[0]
            text_rect.centery = center[1] + radius*0.3

            min_text = self.font_xxxsmall.render(str(min)+data[1], True, text_color)
            min_text_rect = min_text.get_rect()
            min_text_rect.x = center[0]-radius*0.9
            min_text_rect.centery = center[1]+min_text_rect.height

            max_text = self.font_xxxsmall.render(str(max)+data[1], True, text_color)
            max_text_rect = max_text.get_rect()
            max_text_rect.centerx = center[0]
            max_text_rect.centery = center[1]+radius*0.7-max_text_rect.height

            self.screen.blit(min_text,min_text_rect)
            self.screen.blit(max_text,max_text_rect)
            self.screen.blit(text, text_rect)

        angle = self.current_gauges[data_name]/max * degrees
        for i in range(0,round(degrees/15)+1):
            current_angle = degrees/(degrees/15) * i
            factor = 0.8
            if current_angle % 45 == 0:
                factor = 0.7

            color = text_color
            if not invert and current_angle > degrees-degrees/4:
                color = pygame.Color("red")
            if invert and current_angle < degrees/4:
                color = pygame.Color("red")

            base = pointer_tip = (center[0]-(radius*factor*math.cos(math.radians(current_angle))), 
            center[1]-(radius*factor*math.sin(math.radians(current_angle))))
            tip = pointer_tip = (center[0]-(radius*0.9*math.cos(math.radians(current_angle))), 
            center[1]-(radius*0.9*math.sin(math.radians(current_angle))))
            pygame.draw.line(self.screen, color, base, tip, 2)
            pointer_tip = (center[0]-(radius*0.9*math.cos(math.radians(angle))), 
                center[1]-(radius*0.9*math.sin(math.radians(angle))))
            pygame.draw.line(self.screen, pygame.Color("Red"), center, pointer_tip, 3)

        
        
        
    #Endurance screen
    def update_endurance(self):
        if(self.current_window!=4): return
        screen_width = self.screen.get_width()
        screen_height = self.screen.get_height()
        self.draw_gauge(screen_width/6*0.85, (screen_width/6,screen_height/6+screen_height*5/6/3), "Motor", "Motor_Temp", 60)   
        self.draw_gauge(screen_width/6*0.85, (screen_width*3/6,screen_height/6+screen_height*5/6/3), "Speed", "Wheel_Speed", 120, 0, False, 270)   
        self.draw_gauge(screen_width/6*0.85, (screen_width*5/6,screen_height/6+screen_height*5/6/3), "Inverter", "Inverter_Temp", 60)
        self.draw_gauge(screen_width/9*0.85, (screen_width*2/6,screen_height/6+screen_height*7/12), "Battery", "Battery_Temp", 55, 0)
        self.draw_gauge(screen_width/9*0.85, (screen_width*4/6,screen_height/6+screen_height*7/12), "Battery", "Battery_SOC", 100, 0, True)

    def update_laptimes(self):
        if(self.current_window!=999): return
        screen_width = self.screen.get_width()
        screen_height = self.screen.get_height()
        self.draw_gauge((screen_height-screen_height/6)/2*0.85, (screen_width*2/4,screen_height/6+(screen_height-screen_height/6)/2), str(self.data["Wheel_Speed"][0]), "Wheel_Speed", 120, 0, False, 180, True)

    # Shows all the temperatures on the driver page
    def init_driver_data(self):
        self.update_driver_data()

    # Updates the data on the driver page
    def update_driver_data(self):
        if(self.current_window>0): return
        text_display = pygame.Rect((0,0), (self.screen.get_width()*3/5, self.screen.get_height()*6/10))
        text_display.centery = self.screen.get_height()*7/12
        text_display.centerx = self.screen.get_width()/2


        top_box = pygame.Rect((0,text_display.y), (text_display.width*199/200, (text_display.height/3)*96/100))

        top_box.centery = text_display.y + text_display.height/6 + (text_display.height/3)*2/100
        top_box.centerx = text_display.centerx


        bottom_box = pygame.Rect((0,text_display.y), (text_display.width*199/200, (text_display.height/3)*96/100))

        bottom_box.centery = text_display.y + text_display.height*5/6 - (text_display.height/3)*1/100
        bottom_box.centerx = text_display.centerx


        radius = self.screen.get_height()/6
        self.draw_gauge(self.screen.get_height()/6,(top_box.centerx-bottom_box.width*3/8,top_box.centery), "Inverter","Inverter_Temp", 60)
        self.draw_gauge(self.screen.get_height()/6,(top_box.centerx+bottom_box.width*3/8,top_box.centery), "Motor","Motor_Temp", 60)

        self.draw_gauge(self.screen.get_height()/6,(top_box.centerx,top_box.centery), "","Motor_Speed", 3000)
        self.draw_gauge(self.screen.get_height()/6,(bottom_box.centerx,bottom_box.centery), "Speed","Wheel_Speed", 120, 0, False, 270)

        self.draw_gauge(self.screen.get_height()/6,(bottom_box.centerx-bottom_box.width*3/8,bottom_box.centery), "Battery","Battery_Temp", 55)
        self.draw_gauge(self.screen.get_height()/6,(bottom_box.centerx+bottom_box.width*3/8,bottom_box.centery), "Battery","Battery_SOC", 100, 0, True)


    # Initializes the lamps in the top right corner
    def update_lamps(self, lamp=None, value=False):
        #if self.current_window > 0: return
        if lamp != None:
            self.lamps[lamp] = value
        offset = 0
        for i in self.lamps:
            lamp_background_rect = pygame.Rect((0,0), (self.screen.get_width()*1/6, self.screen.get_height()*1/6))
            lamp_background_rect.x = self.screen.get_width() - lamp_background_rect.width - (offset*lamp_background_rect.width)
            pygame.draw.rect(self.screen, BACKGROUND, lamp_background_rect)
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
        if self.current_window != 3: return
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
            self.clock.tick(60)
            #print(self.clock.get_fps())
            for event in pygame.event.get():
                if event.type == UPDATE_APPS:
                    self.lamps["R2D"] = event.data[0]
                    self.data["Throttle"][0] = event.data[1]
                    self.data["Brake_Pressure"][0] = event.data[2]
                elif event.type == UPDATE_BATTERY: 
                    self.data["Battery_SOC"][0] = event.data[0]
                    self.data["Battery_Voltage"][0] = event.data[1]
                elif event.type == UPDATE_BATTERY_TEMP:
                    self.data["Battery_Temp"][0] = event.data
                elif event.type == UPDATE_MOTOR_TEMP:
                    self.data["Motor_Temp"][0] = event.data
                elif event.type == UPDATE_INVERTER_TEMP:
                    self.data["Inverter_Temp"][0] = event.data
                elif event.type == UPDATE_INVERTER_PCB_TEMP:
                    self.data["Inverter_Pcb_Temp"][0] = event.data
                elif event.type == UPDATE_CELL_VOLTAGE:
                    self.data["Cell_Voltage"][0] = event.data
                elif event.type == UPDATE_INVERTER_12V:
                    self.data["Inverter_LV"][0] = event.data
                elif event.type == UPDATE_MOTOR_SPEED:
                    self.data["Motor_Speed"][0] = event.data[0]
                    self.data["Wheel_Speed"][0] = event.data[1]
                elif event.type == pygame.QUIT: 
                    self.can_io.stop_process()
                    sys.exit()
                elif event.type == pygame.KEYDOWN:
                    if event.key == pygame.K_SPACE:
                        self.switch_window()
            self.update_driver_data()
            self.update_lamps()
            self.update_data_screen()
            self.update_endurance()
            pygame.display.flip()
            
