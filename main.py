import gui
import canreader
import can

windows_dev = True
can_io = canreader.AR23CAN(['./cascadia.dbc','./Orion_CANBUS.dbc', './AlignDBC.dbc'],False, "vcan0")

app = gui.UI("Dashbord AR23", 800, 480, can_io)
app.main_loop()