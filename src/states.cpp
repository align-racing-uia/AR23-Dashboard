#include <states.h>

bool btn1 = false;
bool btn2 = false;
bool btn3 = false;
bool btn4 = false;
bool r2d = false;
bool blink = false;

int currentScreen = 0;

// Apps based states
bool r2dState = false;
bool sdcState = false;

// Internal states
int criticalError = 0;
bool updateTopStatus = true;
bool updateBottomStatus = true;
bool updateMiddleStatus = true;
bool updateBrakeStatus = true;
bool updateScreensaver = true;
bool updateDriverStatus = true;

// Inverter states
uint16_t vsmState = 0;
uint16_t inverterState = 0;
int inverterRunFaultCode[] = {0x0, 0x0, 0x0, 0x0};
int inverterPostFaultCode[] = {0x0, 0x0, 0x0, 0x0};
String currentPostFault = "";
bool activePostFault = false;
String currentRunFault = "";
bool activeRunFault = false;
int lowVoltageState = 1200;



// BMS States
int soc = 0;
int dcVoltage = 0;
int packTemp = 0;
int packPower = 0;

// Timestamps
long unsigned int broadcastTimestamp = 0;
long unsigned int inverterTimestamp = 0;
long unsigned int bmsTimestamp = 0;
long unsigned int appsTimestamp = 0;
long unsigned int blinkingTimestamp = 0;