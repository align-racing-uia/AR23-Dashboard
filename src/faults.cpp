#include <faults.h>

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
  updateTopStatus = true;
  updateBottomStatus = true;
  updateMiddleStatus = true;

}

void checkForFaults(){

  // If any bit is set, there is a error.
  if(inverterRunFaultCode[0] | inverterRunFaultCode[1] | inverterRunFaultCode[2] | inverterRunFaultCode[3] > 0){
    mapRunFault();
    if(criticalError != 6){
      updateTopStatus = true;
    }
    criticalError = 6;
  }else if(currentRunFault != ""){
    resetFaultCodes();
  }

  if(inverterPostFaultCode[0] | inverterPostFaultCode[1] | inverterPostFaultCode[2] | inverterPostFaultCode[3] > 0){
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
  }
}
