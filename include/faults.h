#ifndef FAULTS_H_
#define FAULTS_H_

#include <states.h>
#include <inverter.h>


// Should be simplified, but will probably not run much during a single session
void mapRunFault();

void mapPostFault();

void resetFaultCodes();

void checkForFaults();




#endif