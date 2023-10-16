//include statments
#include <iostream>
#include "include/resistance.hpp"

//using namespace as std
using namespace std;

//function declaration dump
int menu();
double calc_resistance();

//main function
int main(){
    menu();
}

//menu function
int menu(){
    cout<<"---------------------------Ram Ohm-----------------------------"<<endl;
    cout<<"---------------------------Version 1.0-------------------------"<<endl;
    cout<<"---------------------------Menu--------------------------------"<<endl;
    cout<<"1.Calculate Resistance"<<endl;
    cout<<"2.Calculate Current"<<endl;
    cout<<"3.Calculate Voltage"<<endl;
    cout<<"4.Calculate Power"<<endl;
    cout<<"5.Exit"<<endl;
    cout<<"Enter your choice: "<<endl;
    int choice;
    cin>>choice;
    if (choice==1){
        double calc_resistance();
    }
}

double calc_resistance(){
    cout<<"Enter the number of resistors: "<<endl;
    int num_resistors;
    cin>>num_resistors;
    

}

