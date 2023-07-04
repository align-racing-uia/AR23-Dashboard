#include <core1.h>

int sharedInverterState = 0;
int sharedVsmState = 0;
int sharedDcVoltage = 0;
int sharedSoc = 0;
bool sharedR2DState = false;
bool sharedSdcState = false;
bool readyForData = false;
int sharedPackTemp = 0;
int sharedPackPower = 0;
int sharedLowVoltageState = 1200;
bool sharedUpdateTopStatus = true;
bool sharedUpdateMiddleStatus = true;
bool sharedUpdateBottomStatus = true;
int sharedInverterRunFaultCode[] = {0x0,0x0,0x0,0x0};
int sharedInverterPostFaultCode[] = {0x0,0x0,0x0,0x0};
