from xml.dom.expatbuilder import theDOMImplementation
import cantools
import can

filter = {
    int("0x00a5", 16): {"can_id": 0x00a5, "can_mask": 0xFFFFFFF, "extended": False, "db": "cascadia"},
    int("0x00a0", 16): {"can_id": 0x00a0, "can_mask": 0xFFFFFFF, "extended": False, "db": "cascadia"},
    int("0x00a2", 16): {"can_id": 0x00a2, "can_mask": 0xFFFFFFF, "extended": False, "db": "cascadia"},
    int("0x06b0", 16): {"can_id": 0x06b0, "can_mask": 0xFFFFFFF, "extended": False, "db": "orion"},
    int("0x06b1", 16): {"can_id": 0x06b1, "can_mask": 0xFFFFFFF, "extended": False, "db": "orion"},
}
    
db = {
    "cascadia": cantools.database.load_file("./cascadia.dbc"),
    "orion": cantools.database.load_file("./Orion_CANBUS.dbc")
}

class CAN_io():
    def __init__(self):
        self.run = True
        self.history = {
            "speed": []
        }

    def bus(self,storage):
        can_bus = can.Bus(interface="socketcan", channel="vcan0", can_filters=list(filter.values()))
        for msg in can_bus:
            if not self.run: break
            
            data = db[filter[msg.arbitration_id]["db"]].decode_message(msg.arbitration_id, msg.data)
            q = []
            found = True
            if(msg.arbitration_id == int("0x00a5",16)):
                self.history["speed"].append(round(data["D2_Motor_Speed"]*0.0314256))
                if(len(self.history["speed"]) > 4):
                    avg = sum(self.history["speed"])/len(self.history["speed"])
                    self.history["speed"] = []
                    storage["speed"] = [True, round(avg)]

            elif(msg.arbitration_id == int("0x06b0",16)):
                storage["soc"] = [True, round(data["Pack_SOC"])]
            elif(msg.arbitration_id == int("0x00a0",16)):
                storage["inverter_temp"] = [True, round((data["D1_Module_A"]+data["D2_Module_B"]+data["D3_Module_C"])/3)]
            elif(msg.arbitration_id == int("0x00a2",16)):
                storage["motor_temp"] = [True, round(data["D3_Motor_Temperature"])]
            elif(msg.arbitration_id == int("0x06b1",16)):
                storage["battery_temp"] = [True, round((data["High_Temperature"]+data["Low_Temperature"])/2)]
                

    def stop(self):
        self.run = False