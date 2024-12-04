/*

A message is sent through the serial bus.
This message is formatted as such.

hex (32bit)
aa bc cd ee

section a: START SIGNAL
    8 bit long section, starting signal will always be 0xC2

section b: FLAGS
    4 bit long section, each bit has a specific meaning

section c: BUTTONS
    8 bit long section, each bit represents a button
    This arrangement allows any number of the 8 available buttons to be pressed at the same time.

section d: EXTRA CALLS
    4 bit long section, read as an integer 0-15
    This section will indicate extra functionality outside of the buttons.
    Only one extra call can be initiated at a time.

section e: ENDING SIGNAL
    8 bit long section, ending signal will always be 0x43

*/


#include <Arduino.h>

// time in ms to wait before checking next button press
#define DELAY 200

const uint8_t BUTTON_PINS[] = {D1, D2, D5, D6, D7}; // digital pins (maximum of 8 can be used with this code)
uint32_t msg = 0; // message sent to pc

void setup() {
  Serial.begin(9600);

  // Set each pin to pullup
  for (int i = 0; i < 5; i++) {
    pinMode(BUTTON_PINS[i], INPUT_PULLUP);
  }

  // Set LED pinmode
  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, HIGH); // Turn onboard LED off
}

void loop() {
  msg = 0x0;  // Clear the message

  // Construct pin section
  for (int i = 0; i < 5; i++) {
    if (!digitalRead(BUTTON_PINS[i])) {
      msg |= 1 << (19-i); // ---- ---- 0000 xxxx xxxx 0000 ---- ----
    }
  }

  // Construct System Call section
  msg |= 0b0000 << 8; // ---- ---- 0000 0000 0000 xxxx ---- ---- // This is currently not doing anything

  // Construct Flag section
  msg |= 0b0000 << 20; //  ---- ---- xxxx 0000 0000 0000 ---- ---- // This is currently not doing anything

  // construct start and finish signals
  msg |= 0xC2000043;

  // Send the message to serial if it is not an empty message
  if (msg != 0xC2000043) {
    // Send start signal
    Serial.write(msg >> 24); // xxxx xxxx ---- ---- ---- ---- 0000 0000

    // Send the message
    Serial.write(msg >> 16 & 0xFF);
    Serial.write(msg >> 8 & 0xFF);

    // Send finish signal
    Serial.write(msg & 0xFF); // 0000 0000 ---- ---- ---- ---- xxxx xxxx
    delay(DELAY); // Delay for a short time 
  }

  // check every 50ms when a message is NOT sent
  // This helps to prevent accidental miss clicks of a button but does not miss events between the DELAY interval
  delay(50); 
}
