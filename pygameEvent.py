import pygame

BACKGROUND = pygame.Color(75,75,75)
CONTAINER = pygame.Color(45,45,45)
TEXT = pygame.Color(75,75,75)
BATTERY = pygame.Color(153, 204, 0) #pygame.Color(51, 204, 51)
HIGHVIS = pygame.Color(255, 153, 0)
DANGER = pygame.Color(204, 51, 0)
ALERT = pygame.Color(255, 153, 0)
OK = pygame.Color(153, 204, 0)#pygame.Color(51, 204, 51)
POWER = pygame.Color(255, 102, 0)
GAUGE_TEXT = pygame.Color(170,170,170)
SHADOW = pygame.Color(35,35,35)

UPDATE_LOG = pygame.USEREVENT
UPDATE_APPS = pygame.USEREVENT+1
UPDATE_BATTERY = pygame.USEREVENT+2
UPDATE_BATTERY_TEMP = pygame.USEREVENT+3
UPDATE_MOTOR_TEMP = pygame.USEREVENT+4
UPDATE_INVERTER_TEMP = pygame.USEREVENT+5
UPDATE_INVERTER_PCB_TEMP = pygame.USEREVENT+6
UPDATE_CELL_VOLTAGE = pygame.USEREVENT+7
UPDATE_INVERTER_12V = pygame.USEREVENT+8
UPDATE_MOTOR_SPEED = pygame.USEREVENT+9