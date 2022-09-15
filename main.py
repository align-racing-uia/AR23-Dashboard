import gui
import canreader

can_io = canreader.AR23CAN(['./cascadia.dbc','./Orion_CANBUS.dbc', './AlignDBC.dbc'],True)
app = gui.UI("Dashbord AR23", 640, 480, can_io)
app.main_loop()