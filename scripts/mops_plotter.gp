set datafile separator ","
set terminal png size 800,600 enhanced font "Helvetica,20"
set output ARG3
set key outside right center
set title ARG2
plot ARG1 using 3:5 with lines notitle
