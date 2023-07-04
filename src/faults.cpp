#include <faults.h>
#include <core1.h>
#include <gui.h>

// Error list
String criticalErrors[] = {"N/A",
                      "AMS Fault", 
                      "Brake Implausibility",
                      "SDC Open",
                      "Inverter Silent",
                      "APPS Silent",
                      "Fault Codes Blinking",
                      "Low System Voltage!!!",
                      "BMS Silent"};

// Should be simplified, but will probably not run much during a single session
void mapRunFault(){
  if(currentRunFault != ""){
    return;
  }
  for(int i=0; i<4; i++){
    for(int y=0; y<8; y++){
      if(inverterRunFaultCode[i] >> y == 1){
        currentRunFault = runFaults[i*8 + y];
        Serial.println(currentRunFault);
        updateMiddleStatus = true;
        return;
      }
    }
  }
}

void mapPostFault(){
  if(currentPostFault != ""){
    return;
  }
  for(int i=0; i<4; i++){
    for(int y=0; y<8; y++){
      if(inverterPostFaultCode[i] >> y == 1){
        currentPostFault = postFaults[i*8 + y];
        Serial.println(currentPostFault);
        updateMiddleStatus = true;
        return;
      }
    }
  }
}

void resetFaultCodes(){
  criticalError = 0;
  currentPostFault = "";
  currentRunFault = "";
  clearScreen(TFT_BLACK);
  updateTopStatus = true;
  updateBottomStatus = true;
  updateMiddleStatus = true;
  updateDriverStatus = true;

}

void checkForFaults(){

  // If any bit is set, there is a error.
  if(sharedInverterRunFaultCode[0] | sharedInverterRunFaultCode[1] | sharedInverterRunFaultCode[2] | sharedInverterRunFaultCode[3] > 0){
    mapRunFault();
    if(criticalError != 6){
      updateTopStatus = true;
    }
    criticalError = 6;
  }else if(currentRunFault != ""){
    resetFaultCodes();
  }

  if(sharedInverterPostFaultCode[0] | sharedInverterPostFaultCode[1] | sharedInverterPostFaultCode[2] | sharedInverterPostFaultCode[3] > 0){
    mapPostFault();
    if(criticalError != 6){
      updateTopStatus = true;
    }

    criticalError = 6;
  }else if(currentPostFault != ""){
    resetFaultCodes();
  }


  // Will only display the first proper error.
  // The last error checked will probably take priority

  if(criticalError < 1){
   // If inverter stops communicating on the CANBUS network, display this error:
    if(millis() - inverterTimestamp > 1000){
      updateTopStatus = criticalError != 4;
      criticalError = 4;
    }

    // If APPS stops communicating on the CANBUS network, display this error:
    if(millis() - appsTimestamp > 2000){
      updateTopStatus = criticalError != 5;
      criticalError = 5;
    }

    if(millis() - bmsTimestamp > 2000){
      updateTopStatus = criticalError != 8;
      criticalError = 8;
    }

    if(sharedLowVoltageState < 1150){
      updateTopStatus = criticalError != 7;
      updateMiddleStatus = criticalError != 7;
      criticalError = 7; 
    }
  }
}
