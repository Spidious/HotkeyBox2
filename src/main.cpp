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

// Time constants
#define DELAY 200         // ms to wait before checking the next button press
#define HEARTBEAT_INTERVAL 1000 // ms between heartbeat messages
#define MAX_MISSES 3      // Number of missed heartbeats before assuming disconnected

const uint8_t BUTTON_PINS[] = {D1, D2, D5, D6, D7}; // Button pins
uint32_t msg = 0;       // Message sent to PC
uint32_t heartbeat_reply = 0x3480002C; // Expected reply for heartbeat

// State tracking
typedef struct {
  bool allowed_to_send; // Can messages (other than heartbeat) be sent?
  bool connected;       // Is the ESP8266 connected to the PC?
  uint8_t miss_count;   // Number of missed heartbeats
} State;

State state = {false, false, 0}; // Initial state

// Function declarations
void send_heartbeat();
void check_serial();

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

  check_serial();

  // Handle button press messages
  if (state.connected && state.allowed_to_send) {
    msg = 0xC2000043;

    // Construct button section
    for (int i = 0; i < 5; i++) {
      if (!digitalRead(BUTTON_PINS[i])) {
        msg |= 1 << (19 - i); // ---- ---- 0000 xxxx xxxx 0000 ---- ----
      }
    }

    // Only send if there's a button pressed
    if (msg != 0xC2000043) {
      for (int i = 3; i >= 0; i--) {
        Serial.write((msg >> (i * 8)) & 0xFF);
      }
      delay(DELAY);
    }
  }

  static unsigned long last_heartbeat = 0;
  unsigned long currentMillis = millis();

  if (currentMillis - last_heartbeat >= HEARTBEAT_INTERVAL) {
    send_heartbeat();
    last_heartbeat = currentMillis;
  }

  // Small delay for debounce
  delay(50);
}


void send_heartbeat() {
  uint32_t default_msg = 0xC2800043;
  if (state.miss_count >= MAX_MISSES) {
    // Send broadcast message
    uint32_t msg = default_msg | 0xFFF00;

    for (int i = 3; i >= 0; i--) {
      Serial.write((msg >> (i * 8)) & 0xFF);
    }
  } else{
    // set the heartbeat bit
    uint32_t msg = default_msg;

    uint32_t err = 0;
    err = state.miss_count;
    msg = (err & 0xFFF) << 8 | msg;

    // Write the msg to serial
    for (int i = 3; i >= 0; i--) {
        Serial.write(msg >> i*8);
    }
  }
}

void check_serial() {
    static unsigned long last_check = 0;
  if (Serial.available() < 4) {
    unsigned long currentMillis = millis();
    if (currentMillis - last_check >= HEARTBEAT_INTERVAL) {
      last_check = currentMillis;
      state.miss_count++;
    }
    return;
  };

  uint32_t received_msg = 0;

  for (int i = 3; i >= 0; i--) {
    received_msg |= ((uint32_t)Serial.read() << (i * 8));
  }

  if (received_msg == heartbeat_reply) {
    state.connected = true;
    state.miss_count = 0;
    state.allowed_to_send = true;
    digitalWrite(LED_BUILTIN, HIGH);
    
    // Send debug message
    uint32_t err = 0xEE0101EE; // heartbeat reply received
    for (int i = 3; i >= 0; i--) {
      Serial.write(err >> i*8);
    }
  } else {
    // static unsigned long last_check = 0;
    unsigned long currentMillis = millis();
    if (currentMillis - last_check >= HEARTBEAT_INTERVAL) {
      last_check = currentMillis;
      if (state.miss_count < MAX_MISSES) {
        state.miss_count++;
        
      } else {
        state.connected = false;
        state.allowed_to_send = false;
        digitalWrite(LED_BUILTIN, LOW);
        // Send debug message
        uint32_t err = 0; // Max misses reached (sending miss count)
        err |= state.miss_count;
        err = err << 8 | 0xEE0000EE;
        for (int i = 3; i >= 0; i--) {
          Serial.write(err >> i*8);
        }
      }
    }
  }
}
