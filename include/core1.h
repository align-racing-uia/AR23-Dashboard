#ifndef CORE1_H_
#define CORE1_H_

extern int sharedInverterState;
extern int sharedCommandedTorque;
extern int sharedVsmState;
extern int sharedDcVoltage;
extern int sharedSoc;
extern int sharedPackTemp;
extern int sharedPackPower;
extern bool sharedR2DState;
extern bool sharedSdcState;
extern bool sharedDeviationState;
extern bool readyForData;
extern int sharedLowVoltageState;
extern bool sharedUpdateTopStatus;
extern bool sharedUpdateMiddleStatus;
extern bool sharedUpdateBottomStatus;
extern bool sharedUpdateDriverStatus;
extern int sharedInverterRunFaultCode[4];
extern int sharedInverterPostFaultCode[4];


#endif
