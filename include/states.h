#ifndef STATES_H_
#define STATES_H_
#include <Arduino.h>
// Button states
extern bool btn1;
extern bool btn2;
extern bool btn3;
extern bool btn4;
extern bool r2d;
extern bool blink;

extern int currentScreen;

// Apps based states
extern bool r2dState;
extern bool sdcState;
extern bool deviationState;

// Internal states
extern int criticalError;
extern bool updateTopStatus;
extern bool updateBottomStatus;
extern bool updateMiddleStatus;
extern bool updateBrakeStatus;
extern bool updateDriverStatus;
extern bool updateScreensaver;

// Inverter states
extern uint16_t vsmState;
extern uint16_t inverterState;
extern int inverterRunFaultCode[4];
extern int inverterPostFaultCode[4];
extern String currentPostFault;
extern bool activePostFault;
extern String currentRunFault;
extern bool activeRunFault;
extern int lowVoltageState;
extern int commandedTorque;



// BMS States
extern int soc;
extern int dcVoltage;
extern int packTemp;
extern int packPower;

// Timestamps
extern long unsigned int broadcastTimestamp;
extern long unsigned int inverterTimestamp;
extern long unsigned int bmsTimestamp;
extern long unsigned int appsTimestamp;
extern long unsigned int blinkingTimestamp;

#endif