
//including header files
#include <iostream>

//declaring std as namespace for easier development
using namespace std;

//function declaration dump
int calcResistance();
int calcCurrent();
int calcVoltage();
int calcPower();
int calcEnergy();

//menu function
int menu(){
	//menu and its options
	cout<<"-----------------------Ram Ohm--------------------------"<<endl;
	cout<<"------------------------V1.0----------------------------"<<endl;
	cout<<"------------------------Menu----------------------------"<<endl;
	cout<<"1.Calculate Resistance"<<endl;
	cout<<"2.Calculate Current"<<endl;
	cout<<"3.Calculate Voltage"<<endl;
	cout<<"4.Calculate Power"<<endl;
	cout<<"5.Calculate Energy"<<endl;
	cout<<"6.Exit"<<endl;
	cout<<"Enter your choice:"<<endl;
	int choice;
	//input for user to enter their choice
	cin>>choice;
	//switch statement for diverting the user to the right function
	switch(choice){
		//if choice is 1, calcResistance() gets called
		case 1:
			calcResistance();
			break;
		//if choice is 2, calcCurrent() gets called
		case 2:
			calcCurrent();
			break;
		//if choice is 3, calcVoltage() gets called			
		case 3:
			calcVoltage();
			break;
		//if choice is 4, calcPower() gets called
		case 4:
			calcPower();
			break;
		//if choice is 5, calcEnergy() gets called
		case 5: 
			calcEnergy();
			break;
		//default case
		default:
			break;
	}
}	


int main()
{
 	menu();   
}

int calcResistance(){
	cout<<"------------------------Calculate Resistance--------------------"<<endl;
	cout<<"Enter 	
