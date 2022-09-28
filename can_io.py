from xml.dom.expatbuilder import theDOMImplementation
import cantools
import can

filter = {
    int("0x00a5", 16): {"can_id": 0x00a5, "can_mask": 0xFFFFFFF, "extended": False, "db": "cascadia"},
    int("0x06b0", 16): {"can_id": 0x06b0, "can_mask": 0xFFFFFFF, "extended": False, "db": "orion"},
    int("0x00a5", 16): {"can_id": 0x00a5, "can_mask": 0xFFFFFFF, "extended": False, "db": "cascadia"}
}
    
db = {
    "cascadia": cantools.database.load_file("./cascadia.dbc"),
    "orion": cantools.database.load_file("./Orion_CANBUS.dbc")
}
class CAN_io():
    def __init__(self):
        self.run = True

    def bus(self,storage):
        can_bus = can.Bus(interface="socketcan", channel="vcan0", can_filters=list(filter.values()))
        for msg in can_bus:
            if not self.run: return
            data = db[filter[msg.arbitration_id]["db"]].decode_message(msg.arbitration_id, msg.data)
            storage.append([msg.arbitration_id, data])
    
    def stop(self):
        self.run = False