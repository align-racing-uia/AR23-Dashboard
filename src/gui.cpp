#include <gui.h>


#define MAX_TORQUE 230 // Nm

TFT_eSPI tft = TFT_eSPI();

void screensaver(){
  // Boot up screen
  tft.fillScreen(TFT_BLACK);
  tft.fillRect(240-logo_width/2, 160-logo_height/2,logo_width, logo_height, TFT_ALIGN);
  tft.drawXBitmap(240-logo_width/2, 160-logo_height/2, logo, logo_width, logo_height, TFT_BLACK);
}

void clearScreen(uint16_t color){
  tft.fillScreen(color);
}

void drawInverterStatus(){
  tft.setTextSize(3);
  // Displaying errors take priority!
  if(criticalError > 0){
    tft.setTextColor(TFT_RED, TFT_BLACK);
    tft.drawString(criticalErrors[criticalError], 18, SCREEN_HEIGHT/10+15);
    return;
  }

  if(sharedInverterState <= 4){
    tft.setTextColor(TFT_YELLOW, TFT_BLACK);
    // Serial.println("Try change to yellow");
    tft.drawString(inverterStates[sharedInverterState], 18, SCREEN_HEIGHT/10+15);
  }else{
    tft.setTextColor(TFT_GREEN, TFT_BLACK);
    // Serial.println("Drawing the green text");
    tft.drawString(inverterStates[sharedInverterState], 18, SCREEN_HEIGHT/10+15);
  }
  
}

void drawR2DStatus(){
  tft.setTextSize(3);
  int color = TFT_RED;
  if(sharedR2DState){
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
  if(sharedVsmState < 5){
    tft.setTextColor(TFT_YELLOW, TFT_BLACK);
    // Serial.println("Try change to yellow");
    tft.drawString(vsmStates[sharedVsmState], 18, (SCREEN_HEIGHT/5*4) + SCREEN_HEIGHT/10+25);
  }else if(vsmState == 5){
    tft.setTextColor(TFT_GREEN, TFT_BLACK);
    // Serial.println("Drawing the green text");
    tft.drawString(vsmStates[sharedVsmState], 18, (SCREEN_HEIGHT/5*4) + SCREEN_HEIGHT/10+25);
  }else{
    tft.setTextColor(TFT_RED, TFT_BLACK);
    // Serial.println("Drawing the green text");
    tft.drawString("Fault Codes Blinking", 18, (SCREEN_HEIGHT/5*4) + SCREEN_HEIGHT/10+25);
  }
}

void drawMiddleStatus(){
  tft.setTextSize(3);
  tft.setTextDatum(BL_DATUM);
  
  u16_t bg_color = TFT_BLACK;
  u16_t lv_color = TFT_GREEN;
  if(sharedLowVoltageState < 1150){
    bg_color = TFT_ORANGE;
    lv_color = TFT_RED;
  }
  tft.setTextColor(TFT_GREEN, TFT_BLACK);
  tft.fillRect(0, SCREEN_HEIGHT/5*1, SCREEN_WIDTH, SCREEN_HEIGHT/10*3, TFT_BLACK);
  tft.drawString("SOC: "+String(sharedSoc)+"%",18, SCREEN_HEIGHT/5+45);
  tft.drawString("TS: "+String(sharedDcVoltage)+"V", 18, SCREEN_HEIGHT/5*2+20);

  tft.setTextColor(lv_color, bg_color);
  tft.drawString("LV: "+String(((float) sharedLowVoltageState) / 100.0)+"V", SCREEN_WIDTH/2+18, SCREEN_HEIGHT/5+45);
  // TODO: Add red text for temperature warning
  tft.setTextColor(TFT_GREEN, TFT_BLACK);
  tft.drawString("TSTMP: "+String(sharedPackTemp) + "C", SCREEN_WIDTH/2+18, SCREEN_HEIGHT/5*2+20);

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

void drawScreensaver() {
  if(updateScreensaver){
    screensaver();
    updateScreensaver = false;
  }
  readyForData = true;
  
}

void drawDriverScreen(){
  u16_t bg_color = TFT_BLACK;
  if(criticalError > 0){
    updateDriverStatus = true;
    if(blink) {
      bg_color = TFT_RED;
    }
    
  }
  if(updateDriverStatus){
    

    if(criticalError > 0){
      clearScreen(bg_color);
      tft.setTextColor(TFT_RED, TFT_BLACK);
      tft.setTextSize(3);
      tft.setTextDatum(MC_DATUM);
      tft.drawString(criticalErrors[criticalError], SCREEN_WIDTH/2, SCREEN_HEIGHT/2);
      tft.setTextDatum(BL_DATUM);
    }else{
      if(sharedUpdateMiddleStatus){
        tft.fillRect(SCREEN_WIDTH/10, SCREEN_HEIGHT/6, SCREEN_WIDTH/10 * 8, SCREEN_HEIGHT/3*2, TFT_BLACK);
      }
      tft.setTextColor(TFT_GREEN, TFT_BLACK);
      tft.setTextSize(8);
      tft.setTextDatum(MC_DATUM);
      tft.drawString("SOC: " + String(sharedSoc) + "%", SCREEN_WIDTH/2, SCREEN_HEIGHT/3);
      tft.drawString("TRQ: " + String(sharedCommandedTorque) + "Nm", SCREEN_WIDTH/2, SCREEN_HEIGHT/3 * 2);
      tft.setTextDatum(BL_DATUM);

    }

  }
  readyForData = true;
}


void drawUI(){
  // No need to sharedUpdate the whole screen for everything, and risking blinky graphics.
  if(readyForData) return;
  if(currentScreen == 0) {
    drawScreensaver();
    return;
  }

  if(currentScreen == 1){
    drawDriverScreen();
    return;
  }

  if(sharedUpdateTopStatus){
    tft.fillRect(5,5, SCREEN_WIDTH-10, SCREEN_HEIGHT/5-10, TFT_BLACK);
    tft.drawRect(0,0, SCREEN_WIDTH, SCREEN_HEIGHT/5, TFT_ALIGN);
    drawInverterStatus();
    sharedUpdateTopStatus = false;
  }

  if(sharedUpdateMiddleStatus){
    drawMiddleStatus();
    sharedUpdateMiddleStatus = false;
  }

  if(sharedUpdateBottomStatus){
    tft.fillRect(0, SCREEN_HEIGHT/5 * 4, SCREEN_WIDTH/6 * 5, SCREEN_HEIGHT/5, TFT_BLACK);
    drawVsmStatus();
    sharedUpdateBottomStatus = false;
    drawR2DStatus();
    drawSDCStatus();
  }
  readyForData = true;
 
}



void initUI(){
    // Configuring screen
    tft.init();
    tft.setRotation(1);
    tft.setTextDatum(BL_DATUM);
  }