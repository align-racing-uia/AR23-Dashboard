#ifndef STATES_H
#define STATES_H
#include <Arduino.h>
// Button states
bool btn1 = false;
bool btn2 = false;
bool btn3 = false;
bool btn4 = false;
bool r2d = false;

// Apps based states
bool r2dState = false;
bool sdcState = false;

// Internal states
int criticalError = 0;
bool updateTopStatus = true;
bool updateBottomStatus = true;
bool updateMiddleStatus = true;

// Inverter states
uint16_t vsmState = 0;
uint16_t inverterState = 0;

// BMS States
int soc = 100;
int dcVoltage = 340;

// Timestamps
long unsigned int broadcastTimestamp = 0;
long unsigned int inverterTimestamp = 0;
long unsigned int appsTimestamp = 0;

#endif