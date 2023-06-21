#include <Arduino.h>
#include <mcp_can.h>

// J1 - Pin 2
// J2 - Pin 3
// J3 - Pin 4
// J4 - Pin 5
// J6 - R2D

#define PIN_J1 2
#define PIN_J2 3
#define PIN_J3 4
#define PIN_J4 5
#define PIN_R2D 6

#define CS_CAN 10

bool fault_flag = false;

MCP_CAN CAN0(CS_CAN);

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);
  if(CAN0.begin(MCP_ANY, CAN_500KBPS, MCP_8MHZ) == CAN_OK){
    Serial.println("MCP2515 Initialized at 500kbps (16MHz)");
  }else{
    Serial.println("MCP2515 Error...");
  }
  CAN0.setMode(MCP_NORMAL);

  pinMode(PIN_J1, INPUT);
  pinMode(PIN_J2, INPUT);
  pinMode(PIN_J3, INPUT);
  pinMode(PIN_J4, INPUT);
  pinMode(PIN_R2D, INPUT);

}

void loop() {
  // put your main code here, to run repeatedly:

  if(digitalRead(PIN_J1)){
    fault_flag = true;
  }

  byte canFrame[1] = {0x0};
  canFrame[0] = digitalRead(PIN_J1);
  canFrame[0] <<= 1;
  canFrame[0] |= digitalRead(PIN_J2);
  canFrame[0] <<= 1;
  canFrame[0] |= digitalRead(PIN_J3);
  canFrame[0] <<= 1;
  canFrame[0] |= digitalRead(PIN_J4);
  canFrame[0] <<= 1;
  canFrame[0] |= digitalRead(PIN_R2D);

  canFrame[0] |= fault_flag << 7;
  

  byte sendFrame = CAN0.sendMsgBuf(0x0E0, 0, 1, canFrame);
  if(sendFrame == CAN_OK) {
    Serial.println("Message sent succesfully");
  }else{
    Serial.println("Error... " + String(sendFrame));
  }
  delay(100);

}