#ifndef CORE1_H_
#define CORE1_H_

extern int sharedInverterState;
extern int sharedVsmState;
extern int sharedDcVoltage;
extern int sharedSoc;
extern int sharedPackTemp;
extern int sharedPackPower;
extern bool sharedR2DState;
extern bool sharedSdcState;
extern bool readyForData;
extern int sharedLowVoltageState;
extern bool sharedUpdateTopStatus;
extern bool sharedUpdateMiddleStatus;
extern bool sharedUpdateBottomStatus;
extern int sharedInverterRunFaultCode[4];
extern int sharedInverterPostFaultCode[4];

#endif
