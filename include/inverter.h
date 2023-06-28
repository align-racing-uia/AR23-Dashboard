
#include <Arduino.h>
#ifndef INVERTER_H
#define INVERTER_H


int inverterRunFaultCode[] = {0x0, 0x0, 0x0, 0x0};
int inverterPostFaultCode[] = {0x0, 0x0, 0x0, 0x0};
String currentPostFault = "";
bool activePostFault = false;
String currentRunFault = "";
bool activeRunFault = false;

char *inverterStates[] = {"Powering on", 
                      "Inverter Stopped", 
                      "Inverter Open Loop", 
                      "Inverter Closed Loop",
                      "Inverter Waiting",
                      "N/A",
                      "N/A",
                      "N/A",
                      "Inverter Idle Run",
                      "Inverter Idle Stopped",
                      "N/A",
                      "N/A",
                      "N/A"};

char *vsmStates[] = {"VSM Start State",
                    "Precharge Init State",
                    "Precharge Active State",
                    "VSM Wait State",
                    "VSM Ready State",
                    "Motor Running State",
                    "Fault Code Blinking State"};

char *postFaults[] = {"Hardware Gate/Desat.Fault",
                      "HW Over-current Fault",
                      "Accelerator Shorted",
                      "Accelerator Open",
                      "Current Sensor Low",
                      "Current Sensor High",
                      "Module Temperature Low",
                      "Module Temperature High",
                      "Control PCB Temperature Low",
                      "Control PCB Temperature High",
                      "GD PCB Temp. Low",
                      "GD PCB Temp. High",
                      "5V Sense Voltage Low",
                      "5V Sense Voltage High",
                      "12V Sense Voltage Low",
                      "12V Sense Voltage High",
                      "2.5V Sense Voltage Low",
                      "2.5V Sense Voltage High",
                      "1.5V Sense Voltage Low",
                      "1.5V Sense Voltage High",
                      "DC Bus Voltage High",
                      "DC Bus Voltage Low",
                      "Pre-charge Timeout",
                      "Pre-charge Voltage Failure",
                      "EEPROM Checksum Invalid",
                      "EEPROM Data Out of Range",
                      "EEPROM Update Required",
                      "DC Bus Over-Voltage on Init",
                      "GDr Initializion",
                      "Reserved",
                      "Brake Shorted",
                      "Brake Open"};   

char *runFaults[] = {"Motor Over-speed Fault",
                  "Over-current Fault",
                  "Over-voltage Fault",
                  "Inverter Over-temperature Fault",
                  "Accelerator Input Shorted Fault",
                  "Accelerator Input Open Fault",
                  "Direction Command Fault",
                  "Inverter Response Time-out Fault",
                  "Hardware Gate/Desat. Fault",
                  "Hardware Over-current Fault",
                  "Under-voltage Fault",
                  "CAN Command Message Lost Fault",
                  "Motor Over-temperature Fault",
                  "Reserved",
                  "Reserved",
                  "Reserved",
                  "Brake Input Shorted Fault",
                  "Brake Input Open Fault",
                  "Module A Over-temperature Fault",
                  "Module B Over-temperature Fault",
                  "Module C Over-temperature Fault",
                  "PCB Over-temperature Fault",
                  "GDB 1 Over-temp. Fault",
                  "GDB 2 Over-temp. Fault",
                  "GDB 3 Over-temp. Fault",
                  "Current Sensor Fault",
                  "GDr Over-Voltage",
                  "Reserved",
                  "Hardware DC Bus Over-voltage Fault",
                  "Reserved",
                  "Resolver Not Connected",
                  "Reserved"};  
#endif