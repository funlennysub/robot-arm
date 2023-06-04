#include <Servo.h>

Servo baseMotor;
Servo elbow;
Servo shoulder;
Servo wrist;
Servo gripper;

int base_pos = 0;
int elbow_pos = 0;
int shoulder_pos = 0;
int wrist_pos = 0;
int gripper_pos = 0;

void setup()
{
  baseMotor.attach(11);
  elbow.attach(10);
  shoulder.attach(9);
  wrist.attach(6);
  gripper.attach(5);

  Serial.begin(9600);

  ipos();
  delay(100);
}

void ipos()
{
  base_pos = 90;
  baseMotor.write(base_pos);
  delay(15);
  elbow_pos = 90;
  elbow.write(elbow_pos);
  delay(15);
  shoulder_pos = 90;
  shoulder.write(shoulder_pos);
  delay(15);
  wrist_pos = 90;
  wrist.write(wrist_pos);
  delay(15);
  gripper_pos = 90;
  gripper.write(gripper_pos);
  delay(15);
}

void left()
{
  base_pos -= 1;
  base_pos = constrain(base_pos, 1, 180);
  baseMotor.write(base_pos);
  delay(15);
}

void right()
{
  base_pos += 1;
  base_pos = constrain(base_pos, 1, 180);
  baseMotor.write(base_pos);
  delay(15);
}

void shoulder_down()
{
  shoulder_pos -= 1;
  shoulder_pos = constrain(shoulder_pos, 1, 180);
  shoulder.write(shoulder_pos);
  delay(15);
}

void shoulder_up()
{
  shoulder_pos += 1;
  shoulder_pos = constrain(shoulder_pos, 1, 180);
  shoulder.write(shoulder_pos);
  delay(15);
}

void elbow_down()
{
  elbow_pos -= 1;
  elbow_pos = constrain(elbow_pos, 1, 180);
  elbow.write(elbow_pos);
  delay(15);
}
void elbow_up()
{
  elbow_pos += 1;
  elbow_pos = constrain(elbow_pos, 1, 180);
  elbow.write(elbow_pos);
  delay(15);
}

void griper_up()
{
  wrist_pos += 1;
  wrist_pos = constrain(wrist_pos, 1, 180);
  wrist.write(wrist_pos);
  delay(15);
}

void griper_down()
{
  wrist_pos -= 1;
  wrist_pos = constrain(wrist_pos, 1, 180);
  wrist.write(wrist_pos);
  delay(15);
}

void gripper_close()
{
  gripper_pos += 1;
  gripper_pos = constrain(gripper_pos, 1, 180);
  gripper.write(gripper_pos);
  delay(15);
}

void gripper_open()
{
  gripper_pos -= 1;
  gripper_pos = constrain(gripper_pos, 1, 180);
  gripper.write(gripper_pos);
  delay(15);
}

void execute(char command)
{
  switch (command)
  {
  case 'L':
    left();
    break;
  case 'I':
    ipos();
    break;
  case 'R':
    right();
    break;
  case 'V':
    shoulder_down();
    break;
  case 'G':
    shoulder_up();
    break;
  case 'B':
    elbow_down();
    break;
  case 'H':
    elbow_up();
    break;
  case 'J':
    griper_up();
    break;
  case 'N':
    griper_down();
    break;
  case 'O':
    gripper_open();
    break;
  case 'C':
    gripper_close();
    break;
  }
  delay(15);
}

void parse_input(String str)
{
  const int len = str.length();
  int carry = 0;

  for (int i = 0; i < len; i++)
  {
    const char chr = str[i];
    if (chr >= '0' && chr <= '9')
    {
      const char nextChr = str[i + 1];
      if (nextChr >= '0' && nextChr <= '9')
      {
        carry = carry * 10 + (chr - '0');
        continue;
      }

      const int times = carry * 10 + (chr - '0');
      carry = 0;

      for (int j = 0; j < times; j++)
      {
        execute(str[i + 1]);
      }

      i += 1;
    }
    else
    {
      execute(str[i]);
    }
  }
}

void loop()
{
  if (Serial.available() > 0)
  {
    String st = Serial.readString();
    parse_input(st);
  }
  delay(10);
}
