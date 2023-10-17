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
        return 0;
    }

//function for calculating resistance
    double calc_resistance(){
        cout<<"---------------------------Calculate Resistance-----------------------------"<<endl;
        cout<<"Enter the number of resistors: "<<endl;
        int num_resistors;
        cin>>num_resistors;
        //dynamic array for storing resistors
        double* resistors = new double[num_resistors];

        //for loop for storing resistors
        for (int i=0; i<num_resistors; i++){
            cout<<"Enter the value of resistor "<<i+1<<": "<<endl;
            cin>>resistors[i];
        }

        //todo: add code for checking if resistors are in series or parallel    
        //variable for storing choice
        int choice;
        cout<<"Are the resistors in series or parallel?"<<endl;
        cout<<"1.Series"<<endl;
        cout<<"2.Parallel"<<endl;
        cout<<"Enter your choice: "<<endl;
        cin>>choice;
        if (choice==1){
            //todo: add code for calculating resistance in series 
            //variable for storing total resistance in series   
            double total_resistance_series=0;
            //for loop for calculating total resistance in series
            for (int i=0; i<num_resistors; i++){
                total_resistance_series+=resistors[i];
            }
            //printing the total resistance in series
            cout<<"The total resistance in series is: "<<total_resistance_series<<endl; 
        }
        else if (choice==2){
            double total_resistance_parallel=0;
            //for loop for calculating total resistance in parallel
            for (int i=0; i<num_resistors; i++){
                total_resistance_parallel+=1/resistors[i];
            }
            //printing the total resistance in parallel
            cout<<"The total resistance in parallel is: "<<total_resistance_parallel<<endl;
        }
        else{
            cout<<"Invalid choice"<<endl;
        }


        //free the memory allocated for the array
        delete[] resistors;
        return 0;
    }//end of calc_resistance function  


