from base64 import decode
import cantools
import re
import time
import pygame
import threading
import math
from pygameEvent import *

#canbus_file = open("./data.trc")
#db1 = cantools.database.load_file('./Orion_CANBUS.dbc')
#db2 = cantools.database.load_file('./cascadia.dbc')

#helper function to convert the hex value from the data, into the byte string can tools wants.
def hexstring_to_bytes(str):
    hex_chars = [*str]
    i=0
    byteList = []
    while i<len(hex_chars):
        byteList.append(int("0x"+hex_chars[i]+hex_chars[i+1],16))
        i += 2  
    return bytes(byteList)


# A class to pretend it is a Can bus I/O
class FakeCan(threading.Thread):
    def __init__(self, canbus_filename,can_reader):
        threading.Thread.__init__(self)
        self.canbus_file = open(canbus_filename)
        self.can_reader = can_reader
        self.running = False
        print("Initialized FakeCan")

    def simulate_can(self):
        msg = self.canbus_file.readline()
        time.sleep(0.00001)
        msg = re.split(r'\s+', msg)
        msg_ar = msg[1].split("#")
        ts = msg[0]
        # converts id from hex to descimal
        id=int(msg_ar[0], 16)
        data = hexstring_to_bytes(msg_ar[1])
        self.can_reader.can_queue.append([id, data, ts])
        return [id, data, ts]

    def stop_process(self):
        self.running = False

    def run(self):
        self.running = True
        print("Starting FakeCan")
        while self.running:
            self.simulate_can()


# A class which reads, and processes the data from the CAN I/O and sends the UI events to deal with the data.
class AR23CAN(threading.Thread):

    def __init__(self,database_filenames,debug=False, filename="formatted_data.log"):
        threading.Thread.__init__(self)
        self.db = []
        filename = "formatted_data.log"
        for db_file in database_filenames:
            self.db.append(cantools.database.load_file(db_file))
        self.can_queue = []
        self.running = False
        self.can_io = None
        print("Initialized AR23CAN")
        if debug:
            self.can_io = FakeCan(filename,self)
        
    def run(self):
        print("Starting AR23CAN")
        self.running = True
        self.can_io.start()
        while self.running:
            if len(self.can_queue)>0:
                #print(len(self.can_queue))
                self.proccess_can_data(*self.can_queue.pop(0))
            else:
                #print("waiting")
                pass

    def stop_process(self):
        self.can_io.stop_process()
        self.running = False

    def proccess_can_data(self,id, data, ts):
        i = 0
        found = False
        #print("Can message from: " + str(id))
        decoded_data = {}
        while i<len(self.db) and found == False:
            try:
                #print(id, data)
                decoded_data = self.db[i].decode_message(id, data)
                #print(decoded_data)
                found = True
            except:
                i+=1 
########## SOC #################
        if(id == 1712):
            #print(decoded_data)
            pygame.event.post(pygame.event.Event(UPDATE_BATTERY, data=[decoded_data["Pack_SOC"], round(decoded_data["Pack_Inst_Voltage"])]))
########## TEMPS ###############
        elif(id == 1713):
            pygame.event.post(pygame.event.Event(UPDATE_BATTERY_TEMP, data=decoded_data["High_Temperature"]))
        elif(id == 162):
            #print(decoded_data)
            pygame.event.post(pygame.event.Event(UPDATE_MOTOR_TEMP, data=round(decoded_data["D3_Motor_Temperature"],1)))
        elif(id == 160):
            temp = (decoded_data["D1_Module_A"] + decoded_data["D2_Module_B"] + decoded_data["D3_Module_C"])/3
            pygame.event.post(pygame.event.Event(UPDATE_INVERTER_TEMP, data=round(temp, 1)))
        elif(id == 377):
            R2D = False
            ShutdownCircuit = False
            if decoded_data["ReadyToDrive"]>0:
                R2D = True
            if decoded_data["ShutdownCircuit"]>0:
                ShutdownCircuit = True

            Throttle = round((decoded_data["Sensor1"] + decoded_data["Sensor2"])/2,1)
            BrakePressure = round(decoded_data["BrakePressure"],1)
            pygame.event.post(pygame.event.Event(UPDATE_APPS, data=[R2D, Throttle, BrakePressure, ShutdownCircuit]))


        elif(id == 403106292):
            pygame.event.post(pygame.event.Event(UPDATE_CELL_VOLTAGE, data=decoded_data["Maximum_Cell_Voltage"]))
        elif(id == 169):
            pygame.event.post(pygame.event.Event(UPDATE_INVERTER_12V, data=round(decoded_data["D4_Reference_Voltage_12_0"],1)))
        elif(id == 165):
            # Wheelspeed = Motorspeed * 0.0314256
            pygame.event.post(pygame.event.Event(UPDATE_MOTOR_SPEED, data=[decoded_data["D2_Motor_Speed"], round(decoded_data["D2_Motor_Speed"]*0.0314256)]))
        else:
            pass
            #print("Not yet implemented")
