from xml.dom.expatbuilder import theDOMImplementation
import cantools
import can

filter = {
    int("0x00a5", 16): {"can_id": 0x00a5, "can_mask": 0xFFFFFFF, "extended": False, "db": "cascadia"},
    int("0x06b0", 16): {"can_id": 0x06b0, "can_mask": 0xFFFFFFF, "extended": False, "db": "orion"},
}
    
db = {
    "cascadia": cantools.database.load_file("./cascadia.dbc"),
    "orion": cantools.database.load_file("./Orion_CANBUS.dbc")
}

class CAN_io():
    def __init__(self):
        self.run = True
        self.last_data = {}

    def bus(self,storage):
        can_bus = can.Bus(interface="socketcan", channel="vcan0", can_filters=list(filter.values()))
        for msg in can_bus:
            if not self.run: return
            
            data = db[filter[msg.arbitration_id]["db"]].decode_message(msg.arbitration_id, msg.data)
            q = []
            found = True
            if(msg.arbitration_id == int("0x00a5",16)):
                q = ["speed", round(data["D2_Motor_Speed"]*0.0314256)]
            else:
                found = False
                
            if found:
                if not q[0] in self.last_data or self.last_data[q[0]] != q[1]:
                    self.last_data[q[0]] = q[1]
                    storage.append(q)
    
    def stop(self):
        self.run = False