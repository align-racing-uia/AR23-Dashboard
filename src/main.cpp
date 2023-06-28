#include "align.h"
#include <Arduino.h>
#include <mcp_can.h>
#include <SPI.h>
#include <TFT_eSPI.h> // This is configured for this exact build.


#include <pico/multicore.h>
#include <pico/mutex.h>
void core1_setup();
void core1_loop();

TFT_eSPI tft = TFT_eSPI();       // Invoke custom library
#define TFT_ALIGN 0xcb42 // The most important color
#define SCREEN_WIDTH 480
#define SCREEN_HEIGHT 320

// All other screen configuration can be found in "./pio/libdeps/pico/TFT_eSPI/User_Setup.h"

// CANBUS Configuration
// NOTE: These pins have to be set in the pins_arduino.h file also, otherwise the default SPI instance uses
// the wrong pins. Workaround suggestions greatly appericated.
// TODO: Add a workaround which doesent include modifying libraries.
#define CAN_SCK 2
#define CAN_MOSI 3
#define CAN_MISO 4
#define CAN_INT 6
#define CAN_CS 5
MCP_CAN CAN0(CAN_CS);

#define CAN_ID 0x0E0

long unsigned int rxId;
unsigned char rxLen = 0;
unsigned char rxBuf[8];

// Configuring buttons
#define PIN_BTN1 27
#define PIN_BTN2 26
#define PIN_BTN3 22
#define PIN_BTN4 28
#define PIN_R2D 14

// Variables
// Inverter based states
uint16_t vsmState = 0;
uint16_t inverterState = 0;
int inverterRunFaultCode[] = {0x0, 0x0, 0x0, 0x0};
int inverterPostFaultCode[] = {0x0, 0x0, 0x0, 0x0};
String currentPostFault = "";
String currentRunFault = "";

char *inverterStates[] = {"Powering on", 
                      "Inverter Stopped", 
                      "Inverter Open Loop", 
                      "Inverter Closed Loop",
                      "Inverter Waiting",
                      "N/A",
                      "N/A",
                      "N/A",
                      "Inverter Idle Run",
                      "Inverter Idle Stopped",
                      "N/A",
                      "N/A",
                      "N/A"};

char *vsmStates[] = {"VSM Start State",
                    "Precharge Init State",
                    "Precharge Active State",
                    "VSM Wait State",
                    "VSM Ready State",
                    "Motor Running State",
                    "Fault Code Blinking State"};

char *postFaults[] = {"Hardware Gate/Desat.Fault",
                      "HW Over-current Fault",
                      "Accelerator Shorted",
                      "Accelerator Open",
                      "Current Sensor Low",
                      "Current Sensor High",
                      "Module Temperature Low",
                      "Module Temperature High",
                      "Control PCB Temperature Low",
                      "Control PCB Temperature High",
                      "GD PCB Temp. Low",
                      "GD PCB Temp. High",
                      "5V Sense Voltage Low",
                      "5V Sense Voltage High",
                      "12V Sense Voltage Low",
                      "12V Sense Voltage High",
                      "2.5V Sense Voltage Low",
                      "2.5V Sense Voltage High",
                      "1.5V Sense Voltage Low",
                      "1.5V Sense Voltage High",
                      "DC Bus Voltage High",
                      "DC Bus Voltage Low",
                      "Pre-charge Timeout",
                      "Pre-charge Voltage Failure",
                      "EEPROM Checksum Invalid",
                      "EEPROM Data Out of Range",
                      "EEPROM Update Required",
                      "DC Bus Over-Voltage on Init",
                      "GDr Initializion",
                      "Reserved",
                      "Brake Shorted",
                      "Brake Open"};   

char *runFaults[] = {"Motor Over-speed Fault",
                  "Over-current Fault",
                  "Over-voltage Fault",
                  "Inverter Over-temperature Fault",
                  "Accelerator Input Shorted Fault",
                  "Accelerator Input Open Fault",
                  "Direction Command Fault",
                  "Inverter Response Time-out Fault",
                  "Hardware Gate/Desat. Fault",
                  "Hardware Over-current Fault",
                  "Under-voltage Fault",
                  "CAN Command Message Lost Fault",
                  "Motor Over-temperature Fault",
                  "Reserved",
                  "Reserved",
                  "Reserved",
                  "Brake Input Shorted Fault",
                  "Brake Input Open Fault",
                  "Module A Over-temperature Fault",
                  "Module B Over-temperature Fault",
                  "Module C Over-temperature Fault",
                  "PCB Over-temperature Fault",
                  "GDB 1 Over-temp. Fault",
                  "GDB 2 Over-temp. Fault",
                  "GDB 3 Over-temp. Fault",
                  "Current Sensor Fault",
                  "GDr Over-Voltage",
                  "Reserved",
                  "Hardware DC Bus Over-voltage Fault",
                  "Reserved",
                  "Resolver Not Connected",
                  "Reserved"};  

 

// BMS States
int soc = 100;
int dcVoltage = 340;

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

// Error list
char *criticalErrors[] = {"N/A",
                      "AMS Fault", 
                      "Brake Implausibility",
                      "SDC Open",
                      "Inverter Silent",
                      "APPS Silent",
                      "Fault Codes Blinking"};

// Timestamps
long unsigned int broadcastTimestamp = 0;
long unsigned int inverterTimestamp = 0;
long unsigned int appsTimestamp = 0;

void screensaver(){
  // Boot up screen
  tft.fillScreen(TFT_BLACK);
  tft.fillRect(240-logo_width/2, 160-logo_height/2,logo_width, logo_height, TFT_ALIGN);
  tft.drawXBitmap(240-logo_width/2, 160-logo_height/2, logo, logo_width, logo_height, TFT_BLACK);
}

void clearScreen(){
  tft.fillScreen(TFT_BLACK);
}

void drawInverterStatus(){
  tft.setTextSize(3);
  // Displaying errors take priority!
  if(criticalError > 0){
    tft.setTextColor(TFT_RED, TFT_BLACK);
    tft.drawString(criticalErrors[criticalError], 18, SCREEN_HEIGHT/10+15);
    return;
  }

  if(inverterState <= 4){
    tft.setTextColor(TFT_YELLOW, TFT_BLACK);
    // Serial.println("Try change to yellow");
    tft.drawString(inverterStates[inverterState], 18, SCREEN_HEIGHT/10+15);
  }else{
    tft.setTextColor(TFT_GREEN, TFT_BLACK);
    // Serial.println("Drawing the green text");
    tft.drawString(inverterStates[inverterState], 18, SCREEN_HEIGHT/10+15);
  }
  
}

void drawR2DStatus(){
  tft.setTextSize(3);
  int color = TFT_RED;
  if(r2dState){
    color = TFT_GREEN;
  }
  tft.fillRect(((SCREEN_WIDTH / 6) * 5) + 1, SCREEN_HEIGHT / 5 * 4 + 1, (SCREEN_WIDTH / 6) - 2, (SCREEN_HEIGHT / 5)- 2, color);
  tft.setTextDatum(BC_DATUM);
  tft.setTextColor(TFT_BLACK, color);
  tft.drawString("R2D", ((SCREEN_WIDTH / 6) * 5) + (SCREEN_WIDTH / 12), SCREEN_HEIGHT/5*4 + SCREEN_HEIGHT/10+15);
  tft.setTextDatum(BL_DATUM);

}

void drawSDCStatus(){
  tft.setTextSize(3);
  int color = TFT_RED;
  if(sdcState){
    color = TFT_GREEN;
  }
  tft.fillRect(((SCREEN_WIDTH / 6) * 5) + 1,1+(SCREEN_HEIGHT/5*3), (SCREEN_WIDTH / 6) - 2, (SCREEN_HEIGHT / 5) - 2, color);
  tft.setTextDatum(BC_DATUM);
  tft.setTextColor(TFT_BLACK, color);
  tft.drawString("SDC", ((SCREEN_WIDTH / 6) * 5) + (SCREEN_WIDTH / 12), 1+(SCREEN_HEIGHT/5*3)+SCREEN_HEIGHT/10+15);
  tft.setTextDatum(BL_DATUM);

}

void drawVsmStatus(){
  tft.setTextSize(2);
  if(vsmState < 5){
    tft.setTextColor(TFT_YELLOW, TFT_BLACK);
    // Serial.println("Try change to yellow");
    tft.drawString(vsmStates[vsmState], 18, (SCREEN_HEIGHT/5*4) + SCREEN_HEIGHT/10+25);
  }else if(vsmState == 5){
    tft.setTextColor(TFT_GREEN, TFT_BLACK);
    // Serial.println("Drawing the green text");
    tft.drawString(vsmStates[vsmState], 18, (SCREEN_HEIGHT/5*4) + SCREEN_HEIGHT/10+25);
  }else{
    tft.setTextColor(TFT_RED, TFT_BLACK);
    // Serial.println("Drawing the green text");
    tft.drawString("Fault Codes Blinking", 18, (SCREEN_HEIGHT/5*4) + SCREEN_HEIGHT/10+25);
  }
}

void drawMiddleStatus(){
  tft.setTextSize(3);
  tft.setTextDatum(BL_DATUM);
  tft.setTextColor(TFT_GREEN, TFT_BLACK);
  tft.fillRect(0, SCREEN_HEIGHT/5*1, SCREEN_WIDTH/2, SCREEN_HEIGHT/10*3, TFT_BLACK);
  tft.drawString("SOC: "+String(soc)+"%",18, SCREEN_HEIGHT/5*1+45);
  tft.drawString("TS DC: "+String(dcVoltage)+"V", 18, SCREEN_HEIGHT/5*2+20);
  tft.setTextColor(TFT_RED, TFT_BLACK);
  tft.setTextDatum(TL_DATUM);
  tft.setTextSize(2);
  tft.fillRect(0, SCREEN_HEIGHT/2, SCREEN_WIDTH/6*5, SCREEN_HEIGHT/10*3, TFT_BLACK);
  
  if(currentRunFault != String("")){
    tft.drawString("Run Faults:",18, SCREEN_HEIGHT/2+5);
    tft.drawString(currentRunFault,18, SCREEN_HEIGHT/2+25);
  }
  if(currentPostFault != String("")){
    tft.drawString("Post Faults:",18, SCREEN_HEIGHT/2+45);
    tft.drawString(currentPostFault,18, SCREEN_HEIGHT/2+65);
  }

  tft.setTextDatum(BL_DATUM);
}


void drawUI(){
  // No need to update the whole screen for everything, and risking blinky graphics.
  if(updateTopStatus){
    tft.fillRect(5,5, SCREEN_WIDTH-10, SCREEN_HEIGHT/5-10, TFT_BLACK);
    tft.drawRect(0,0, SCREEN_WIDTH, SCREEN_HEIGHT/5, TFT_ALIGN);
    drawInverterStatus();
    updateTopStatus = false;
  }

  if(updateMiddleStatus){
    drawMiddleStatus();
    updateMiddleStatus = false;
  }

  if(updateBottomStatus){
    tft.fillRect(0, SCREEN_HEIGHT/5 * 4, SCREEN_WIDTH, SCREEN_HEIGHT/5, TFT_BLACK);
    drawVsmStatus();
    updateBottomStatus = false;
    drawR2DStatus();
    drawSDCStatus();
  }
  
}
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




void canRx(){
  
  if(!digitalRead(CAN_INT)){
    CAN0.readMsgBuf(&rxId, &rxLen, rxBuf);
  }else{
    return;
  }
  int oldState = 0;
  switch(rxId){
    case 0x0AA:
      // Handle VSM State
      oldState = vsmState;
      vsmState = rxBuf[0];
      updateBottomStatus = updateBottomStatus || vsmState != oldState;

      oldState = inverterState;
      inverterState = rxBuf[2];

      updateTopStatus = updateTopStatus || oldState != inverterState;
      
      inverterTimestamp = millis();
      break;

    case 0x0A7:
      oldState = dcVoltage;
      dcVoltage = rxBuf[1] << 8;
      dcVoltage |= rxBuf[0];
      dcVoltage /= 10;
      updateMiddleStatus = updateMiddleStatus || dcVoltage != oldState;
      break;

    case 0x6B0:
      oldState = soc;
      soc = rxBuf[4]/2;
      updateMiddleStatus = updateMiddleStatus || soc != oldState;
      break;

    case 0x0AB:
      inverterPostFaultCode[0] = rxBuf[0];
      inverterPostFaultCode[1] = rxBuf[1];
      inverterPostFaultCode[2] = rxBuf[2];
      inverterPostFaultCode[3] = rxBuf[3];
      inverterRunFaultCode[0] = rxBuf[4];
      inverterRunFaultCode[1] = rxBuf[5];
      inverterRunFaultCode[2] = rxBuf[6];
      inverterRunFaultCode[3] = rxBuf[7];

      break;

    case 0x0E1:
      oldState = r2dState;
      r2dState = 0x2 & rxBuf[3];
      updateBottomStatus = updateBottomStatus || oldState != r2dState;

      oldState = sdcState;
      sdcState = 0x4 & rxBuf[3];
      updateBottomStatus = updateBottomStatus || oldState != sdcState;
      appsTimestamp = millis();
      break;
    default:
      Serial.println("Recieved unknown message..");
      return;
  }

  
}

void canTx(){
  byte canFrame[2] = {0x0, 0x0};
  canFrame[0] = btn1;
  canFrame[0] <<= 1;
  canFrame[0] |= btn2;
  canFrame[0] <<= 1;
  canFrame[0] |= btn3;
  canFrame[0] <<= 1;
  canFrame[0] |= btn4;
  canFrame[0] <<= 1;
  canFrame[0] |= r2d;

  // Byte 2 is logged error state.
  canFrame[1] = criticalError;

  CAN0.sendMsgBuf(CAN_ID, 0, 2, canFrame);
  // Serial.println("Frame sent.");
}

void setup(void) {
  Serial.begin(9600);
  SPI.begin();
  delay(1000);
  // Configuring screen
  tft.init();
  tft.setRotation(1);
  tft.setTextDatum(BL_DATUM);

  screensaver();
  
  // Initializing CANBUS communication
  // Initialize MCP2515 running at 16MHz with a baudrate of 500kb/s and the masks and filters disabled.
  pinMode(CAN_INT, INPUT);
  pinMode(CAN_CS, OUTPUT);

  
  if(CAN0.begin(MCP_STDEXT, CAN_500KBPS, MCP_16MHZ) == CAN_OK){
    Serial.println("MCP2515 Initialized at 500kbps (16MHz)");
  }else{
    Serial.println("MCP2515 Error...");
  }
  // All filters and masks has to be set for it to work properly.
  CAN0.init_Mask(0, 0x03FF0000);
  CAN0.init_Mask(1, 0x06FF0000);
  CAN0.init_Filt(0, 0x00A70000); //+ Owned by mask 0
  CAN0.init_Filt(1, 0x00AA0000); //+
  CAN0.init_Filt(2, 0x06B00000); //- Owned by mask 1
  CAN0.init_Filt(3, 0x00E10000); //-
  CAN0.init_Filt(4, 0x00AB0000); //-
  CAN0.init_Filt(5, 0x00AA0000); //-


  CAN0.setMode(MCP_NORMAL);   // Change to normal mode to allow messages to be transmitted


  pinMode(PIN_BTN1, INPUT);
  pinMode(PIN_BTN2, INPUT);
  pinMode(PIN_BTN3, INPUT);
  pinMode(PIN_BTN4, INPUT);
  pinMode(PIN_R2D, INPUT);

  clearScreen();
  inverterTimestamp = millis();
  appsTimestamp = millis();

  // Launching the second core of the pico:
  // multicore_launch_core1(core1_loop);

}

void loop() {
  // Gathering information
  canRx();
  btn1 = digitalRead(PIN_BTN1);
  btn2 = digitalRead(PIN_BTN2);
  btn3 = digitalRead(PIN_BTN3);
  btn4 = digitalRead(PIN_BTN4);
  r2d = digitalRead(PIN_R2D);

  checkForFaults();
  drawUI();
  //Resetting fault codes can be practical during testing
  if(btn3){
    resetFaultCodes();
  }

  if(millis() - broadcastTimestamp > 100){
    canTx();
    broadcastTimestamp = millis();
  }

  // TODO: Display state of charge
  


  delay(5);

}

void core1_loop(){
  // TODO: Move some heavy lifting over here 
}

